import * as THREE from 'three';
import { PolytopeWasm, type ArrayRef } from '$lib/pkg/rs';
import { memory } from '$lib/pkg/rs_bg.wasm';
import CustomShaderMaterial from 'three-custom-shader-material/vanilla';

const VERTEXSHADER_COMMON = `
vec3 projectTo3D(vec4 pos) {
	vec3 xyz = pos.xyz;
	float d = 2.0 / (pos.w + 2.0);
	xyz *= d;
	return xyz;
}

float depth(float w) {
	return clamp((w + 1.0) / 2.0, 0.0, 1.0);
}
`;

const VERTEX_VERTEXSHADER = `
uniform float depthScaling;

varying float vDepth;

attribute vec4 pos;

${VERTEXSHADER_COMMON}

void main() {
	float myDepth = depth(pos.w);
	csm_Position = position;
	csm_Position *= mix( 1.0 + depthScaling, 1.0 - depthScaling, myDepth );
	csm_Position += projectTo3D(pos);
	vDepth = myDepth;
}
`;

const VERTEX_FRAGMENTSHADER = `
uniform vec3 nearColor;
uniform vec3 farColor;

varying float vDepth;

void main() {
	csm_DiffuseColor = vec4( mix(nearColor, farColor, vDepth), opacity );
}
`;

const EDGE_VERTEXSHADER = `
uniform float depthScaling;

varying vec2 vUv;
varying float vDepth1;
varying float vDepth2;

attribute vec4 posFrom;
attribute vec4 posTo;

${VERTEXSHADER_COMMON}

mat3 lookAt(vec3 eye, vec3 at) {
	vec3 localUp = vec3(0.0, 1.0, 0.0);
	vec3 fwd = normalize(eye - at);

	float upDot = dot(localUp, fwd);
	bool isUp = abs(1.0 - abs(upDot)) < 0.00001;

	localUp.z += float(isUp) * 1.0;
	localUp.y += float(isUp) * -1.0;

	vec3 right = normalize(cross(localUp, fwd));
	vec3 up = normalize(cross(fwd, right));

	return mat3(right, up, fwd);
}

void main() {
	vUv = vec3( uv, 1 ).xy;

	vec3 from = projectTo3D(posFrom);
	vec3 to = projectTo3D(posTo);
	float depth1 = depth(posFrom.w);
	float depth2 = depth(posTo.w);

	csm_Position = position;
	float scaleFactor = mix( 1.0 + depthScaling, 1.0 - depthScaling, depth1 + (depth2 - depth1) * vUv.y );
	csm_Position.x *= scaleFactor;
	csm_Position.y *= scaleFactor;
	csm_Position.z *= distance(from, to);

	mat3 lookAtMatrix = lookAt(from, to);
	csm_Position = lookAtMatrix * csm_Position;
	csm_Normal = lookAtMatrix * normal;

	csm_Position += from;

	vDepth1 = depth1;
	vDepth2 = depth2;
}
`;

const EDGE_FRAGMENTSHADER = `
uniform vec3 nearColor;
uniform vec3 farColor;

varying float vDepth1;
varying float vDepth2;
varying vec2 vUv;

void main() {
	csm_DiffuseColor = vec4( mix( nearColor, farColor, vDepth1 + (vDepth2 - vDepth1) * vUv.y ), opacity );
}
`;

type Uniform<T> = { value: T };

///
export class Polytope {
	wasm: PolytopeWasm;

	scene: THREE.Scene;
	vertex: {
		mesh: THREE.Mesh;
	};
	edge: {
		mesh: THREE.Mesh;
	};

	uniforms: {
		nearColor: Uniform<THREE.Color>;
		farColor: Uniform<THREE.Color>;
		depthScaling: Uniform<number>;
	};
	// face: {
	//     mesh: THREE.Mesh;
	//     positionAttribute: THREE.Float32BufferAttribute;
	// }

	constructor(scene: THREE.Scene, diagram: string) {
		this.scene = scene;

		this.wasm = new PolytopeWasm(diagram);
		this.wasm.update();

		const thickness = this.wasm.get_ideal_thickness();

		// A couple of advanced tricks are used in this code:

		// trick #1:
		// We patch the vertex and framgment shaders of the PhongMaterial to obtain extra
		// functionalities while benefiting from the existing shader functionality. These
		// new functionalities are:
		// 1. Coloring of the edges/vertices in a gradient corresponding to a new
		//    "depth" attribute.
		// 2. Scaling of the edges/vertices based on this depth, so that "deeper"
		//    objects appear smaller.
		// The depth attribute in question is related to the projected coorinate of the
		// polytope.

		// trick #2:
		// Since the instance matrices and buffer atributes are all calculated in wasm,
		// copying them to js to then copy them to the gpu makes no sense. So instead
		// we make Float32Array views over the portion of the wasm modules linear memory
		// containing the relevant buffer attribute. Doing this by making use of the ability
		// to pass pointers from the wasm module.

		// TODO: Make these modifiable

		// future plan:
		// rotation_matrix: a 4D matrix representing the objects rotation,
		// the object can now be rotated entirely in 4d space
		const extraUniforms = {
			nearColor: { value: new THREE.Color().setHSL(60 / 360, 1, 0.5) },
			farColor: { value: new THREE.Color().setHSL(0 / 360, 1, 0.5) },
			depthScaling: { value: 0.2 }
		};

		/// Create vertices
		const vertexGeometry = new THREE.InstancedBufferGeometry().copy(
			new THREE.SphereGeometry(thickness, 16, 16)
		);

		const vertexMaterial = new CustomShaderMaterial({
			baseMaterial: THREE.MeshPhongMaterial,
			vertexShader: VERTEX_VERTEXSHADER,
			fragmentShader: VERTEX_FRAGMENTSHADER,
			uniforms: extraUniforms,
			shininess: 100
		});

		const dataRefs = this.wasm.get_render_data_refs();

		vertexGeometry.instanceCount = Math.floor(dataRefs.vertex_pos.length / 4);
		const vertexPosAttribute = wasmInstancedBufferAttribute(dataRefs.vertex_pos, 4);
		vertexGeometry.setAttribute('pos', vertexPosAttribute);
		const vertexMesh = new THREE.Mesh(vertexGeometry, vertexMaterial);
		// wasmInstancedMesh(dataRefs.vertex_instances, vertexGeometry, vertexMaterial);
		this.scene.add(vertexMesh);
		this.vertex = {
			mesh: vertexMesh
		};

		// edges

		const edgeGeometry = new THREE.InstancedBufferGeometry().copy(
			new THREE.CylinderGeometry(thickness, thickness, 1, 16, 1, true)
				.rotateX(-Math.PI / 2)
				.translate(0, 0, -0.5)
		);
		const edgeMaterial = new CustomShaderMaterial({
			baseMaterial: THREE.MeshPhongMaterial,
			vertexShader: EDGE_VERTEXSHADER,
			fragmentShader: EDGE_FRAGMENTSHADER,
			uniforms: extraUniforms,
			shininess: 100
		});

		edgeGeometry.instanceCount = Math.floor(dataRefs.edge_pos_from.length / 4);

		const edgeFromTwoAttribute = wasmInstancedBufferAttribute(dataRefs.edge_pos_from, 4);
		edgeGeometry.setAttribute('posFrom', edgeFromTwoAttribute);
		const edgeToTwoAttribute = wasmInstancedBufferAttribute(dataRefs.edge_pos_to, 4);
		edgeGeometry.setAttribute('posTo', edgeToTwoAttribute);

		const edgeMesh = new THREE.Mesh(edgeGeometry, edgeMaterial);
		// const edgeMesh = wasmInstancedMesh(dataRefs.edge_instances, edgeGeometry, edgeMaterial);
		this.scene.add(edgeMesh);
		this.edge = {
			mesh: edgeMesh
		};
		this.uniforms = extraUniforms;
	}

	update() {
		this.wasm.update();
	}

	rotate(theta: number) {
		this.wasm.rotate(theta, 0, 3);
		// this.uniforms.scaleMat.value[0] += 0.01;
	}
}

function wasmTypedArray(wasmArray: ArrayRef): THREE.TypedArray {
	switch (wasmArray.item_type) {
		case 'Float32':
			return new Float32Array(memory.buffer, wasmArray.pointer, wasmArray.length);
		case 'UInt16':
			return new Uint16Array(memory.buffer, wasmArray.pointer, wasmArray.length);
		case 'UInt32':
			return new Uint32Array(memory.buffer, wasmArray.pointer, wasmArray.length);
	}
}

function wasmInstancedBufferAttribute(
	wasmArray: ArrayRef,
	item_size: number,
	normalized?: boolean,
	meshPerAtribute?: number
): THREE.InstancedBufferAttribute {
	return new THREE.InstancedBufferAttribute(
		wasmTypedArray(wasmArray),
		item_size,
		normalized,
		meshPerAtribute
	);
}

// function wasmBA(wasmArray: WasmArray, item_size: number): THREE.BufferAttribute {
// 	return new THREE.BufferAttribute(wasmTypedArray(wasmArray), item_size);
// }

// function wasmInstancedMesh(
// 	wasmArray: ArrayRef,
// 	geometry: THREE.BufferGeometry,
// 	material: THREE.Material
// ): THREE.InstancedMesh {
// 	const mesh = new THREE.InstancedMesh(geometry, material, Math.floor(wasmArray.length / 16));
// 	mesh.instanceMatrix = wasmInstancedBufferAttribute(wasmArray, 16);
// 	return mesh;
// }
