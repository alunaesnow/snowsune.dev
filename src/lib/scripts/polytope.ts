import * as THREE from 'three';
import { PolytopeWasm, type ArrayRef } from '$lib/pkg/rs';
import { memory } from '$lib/pkg/rs_bg.wasm';

///
export class Polytope {
	wasm: PolytopeWasm;

	scene: THREE.Scene;
	vertex: {
		mesh: THREE.Mesh;
		depthAttribute: THREE.InstancedBufferAttribute;
		positionAttribute: THREE.InstancedBufferAttribute;
	};
	edge: {
		mesh: THREE.InstancedMesh;
		depth1Attribute: THREE.InstancedBufferAttribute;
		depth2Attribute: THREE.InstancedBufferAttribute;
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
		//
		const extraUniforms = {
			nearColor: { value: new THREE.Color().setHSL(60 / 360, 1, 0.5) },
			farColor: { value: new THREE.Color().setHSL(0 / 360, 1, 0.5) },
			depthScaling: { value: 0.2 }
		};

		/// Create vertices
		const vertexGeometry = new THREE.InstancedBufferGeometry().copy(
			new THREE.SphereGeometry(thickness, 16, 16)
		);
		const vertexMaterial = new THREE.MeshPhongMaterial({ shininess: 100 });
		vertexMaterial.onBeforeCompile = (shader) => {
			shader.uniforms = { ...shader.uniforms, ...extraUniforms };
			shader.vertexShader = shader.vertexShader
				.replace(
					'#define PHONG',
					`#define PHONG
          uniform float depthScaling;
          varying float vDepth;`
				)
				.replace(
					'#include <common>',
					`#include <common>
          attribute float depth;
		  attribute vec3 vertPos;`
				)
				.replace(
					'#include <project_vertex>',
					`transformed *= mix( 1.0 + depthScaling, 1.0 - depthScaling, depth );
					transformed += vertPos;
          #include <project_vertex>
          vDepth = depth;`
				);

			shader.fragmentShader = `
        uniform vec3 nearColor;
        uniform vec3 farColor;
        varying float vDepth;
        ${shader.fragmentShader.replace(
					'vec4 diffuseColor = vec4( diffuse, opacity );',
					'vec4 diffuseColor = vec4( mix(nearColor, farColor, vDepth), opacity );'
				)}
        `;
		};

		const dataRefs = this.wasm.get_render_data_refs();

		vertexGeometry.instanceCount = dataRefs.vertex_depths.length;
		const vertexDepthAttribute = wasmInstancedBufferAttribute(dataRefs.vertex_depths, 1);
		vertexGeometry.setAttribute('depth', vertexDepthAttribute);
		const vertexPositionAttribute = wasmInstancedBufferAttribute(
			dataRefs.vertex_positions,
			3,
			false,
			1
		);
		vertexGeometry.setAttribute('vertPos', vertexPositionAttribute);
		const vertexMesh = new THREE.Mesh(vertexGeometry, vertexMaterial);
		// wasmInstancedMesh(dataRefs.vertex_instances, vertexGeometry, vertexMaterial);
		this.scene.add(vertexMesh);
		this.vertex = {
			mesh: vertexMesh,
			depthAttribute: vertexDepthAttribute,
			positionAttribute: vertexPositionAttribute
		};

		// edges
		// const edgeData = getEdgesFromFaces(faces);
		const edgeGeometry = new THREE.CylinderGeometry(thickness, thickness, 1, 16, 1, true);
		edgeGeometry.rotateX(-Math.PI / 2);
		edgeGeometry.translate(0, 0, -0.5);
		const edgeMaterial = new THREE.MeshPhongMaterial({
			shininess: 100,
			transparent: false
		});
		edgeMaterial.defines = { USE_UV: '' };
		edgeMaterial.onBeforeCompile = (shader) => {
			shader.uniforms = { ...shader.uniforms, ...extraUniforms };
			shader.vertexShader = shader.vertexShader
				.replace(
					'#define PHONG',
					`#define PHONG
          uniform float depthScaling;
          varying float vDepth1;
          varying float vDepth2;`
				)
				.replace(
					'#include <common>',
					`#include <common>
          attribute float depth1;
          attribute float depth2;`
				)
				.replace(
					'#include <project_vertex>',
					`float scaleFactor = mix( 1.0 + depthScaling, 1.0 - depthScaling, depth1 + (depth2 - depth1) * vUv.y );
          transformed.x *= scaleFactor;
          transformed.y *= scaleFactor;
          #include <project_vertex>
          vDepth1 = depth1;
          vDepth2 = depth2;`
				);

			shader.fragmentShader = ` 
        uniform vec3 nearColor;
        uniform vec3 farColor;
        varying float vDepth1;
        varying float vDepth2;
        ${shader.fragmentShader.replace(
					'vec4 diffuseColor = vec4( diffuse, opacity );',
					'vec4 diffuseColor = vec4( mix( nearColor, farColor, vDepth1 + (vDepth2 - vDepth1) * vUv.y ), opacity );'
				)}
        `;
		};

		const edgeDepth1Attribute = wasmInstancedBufferAttribute(dataRefs.edge_depth1s, 1);
		edgeGeometry.setAttribute('depth1', edgeDepth1Attribute);
		const edgeDepth2Attribute = wasmInstancedBufferAttribute(dataRefs.edge_depth2s, 1);
		edgeGeometry.setAttribute('depth2', edgeDepth2Attribute);
		const edgeMesh = wasmInstancedMesh(dataRefs.edge_instances, edgeGeometry, edgeMaterial);
		this.scene.add(edgeMesh);
		this.edge = {
			mesh: edgeMesh,
			depth1Attribute: edgeDepth1Attribute,
			depth2Attribute: edgeDepth2Attribute
		};
	}

	update() {
		this.wasm.update();

		// Need to mark the update flags on the instance matrices
		// and buffer attributes, else the changes wont be uploaded
		// to the GPU
		// this.vertex.mesh.instanceMatrix.needsUpdate = true;
		this.vertex.positionAttribute.needsUpdate = true;
		this.vertex.depthAttribute.needsUpdate = true;

		this.edge.mesh.instanceMatrix.needsUpdate = true;
		this.edge.depth1Attribute.needsUpdate = true;
		this.edge.depth2Attribute.needsUpdate = true;
	}

	rotate(theta: number) {
		this.wasm.rotate(theta, 0, 3);
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

function wasmInstancedMesh(
	wasmArray: ArrayRef,
	geometry: THREE.BufferGeometry,
	material: THREE.Material
): THREE.InstancedMesh {
	const mesh = new THREE.InstancedMesh(geometry, material, Math.floor(wasmArray.length / 16));
	mesh.instanceMatrix = wasmInstancedBufferAttribute(wasmArray, 16);
	return mesh;
}
