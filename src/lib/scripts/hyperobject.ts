import * as THREE from 'three';
import { F32ArrSizedPointer, HyperObjectWasm } from '$lib/pkg/rs';
import { memory } from '$lib/pkg/rs_bg.wasm';

export class HyperObject {
	wasm: HyperObjectWasm | undefined;

	scene: THREE.Scene;
	vertex:
		| undefined
		| {
				mesh: THREE.InstancedMesh;
				depthAttribute: THREE.InstancedBufferAttribute;
		  };
	edge:
		| undefined
		| {
				mesh: THREE.InstancedMesh;
				depth1Attribute: THREE.InstancedBufferAttribute;
				depth2Attribute: THREE.InstancedBufferAttribute;
		  };
	// face: {
	//     mesh: THREE.Mesh;
	//     positionAttribute: THREE.Float32BufferAttribute;
	// }

	constructor(scene: THREE.Scene) {
		this.scene = scene;
	}

	loadPoly(diagram: string) {
		this.wasm = new HyperObjectWasm(diagram);
		this.wasm.update();

		// TODO: Make these modifiable
		const extraUniforms = {
			nearColor: { value: new THREE.Color().setHSL(60 / 360, 1, 0.5) },
			farColor: { value: new THREE.Color().setHSL(0 / 360, 1, 0.5) },
			depthScaling: { value: 0.2 }
		};

		/// Create vertices
		const vertexGeometry = new THREE.SphereGeometry(0.035, 16, 16);
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
			  attribute float depth;`
				)
				.replace(
					'#include <project_vertex>',
					`transformed *= mix( 1.0 + depthScaling, 1.0 - depthScaling, depth );
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

		const [depths, instances] = this.wasm.get_arr_pointers();

		const vertexDepthAttribute = f32WasmIBA(depths, 1);
		vertexGeometry.setAttribute('depth', vertexDepthAttribute);
		const vertexMesh = patchedInstancedMesh(vertexGeometry, vertexMaterial, instances);
		this.scene.add(vertexMesh);
		this.vertex = { mesh: vertexMesh, depthAttribute: vertexDepthAttribute };

		depths.free();
		instances.free();

		// edges
		// const edgeData = getEdgesFromFaces(faces);
		const edgeGeometry = new THREE.CylinderGeometry(0.035, 0.035, 1, 16, 1, true);
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

		const [depth1s, depth2s, instances2] = this.wasm.get_edge_mat_pointers();

		const edgeDepth1Attribute = f32WasmIBA(depth1s, 1);
		edgeGeometry.setAttribute('depth1', edgeDepth1Attribute);
		const edgeDepth2Attribute = f32WasmIBA(depth2s, 1);
		edgeGeometry.setAttribute('depth2', edgeDepth2Attribute);
		const edgeMesh = patchedInstancedMesh(edgeGeometry, edgeMaterial, instances2);
		this.scene.add(edgeMesh);
		this.edge = {
			mesh: edgeMesh,
			depth1Attribute: edgeDepth1Attribute,
			depth2Attribute: edgeDepth2Attribute
		};

		depth1s.free();
		depth2s.free();
		instances2.free();
	}

	update() {
		this.wasm?.update();

		if (this.vertex) {
			this.vertex.mesh.instanceMatrix.needsUpdate = true;
			this.vertex.depthAttribute.needsUpdate = true;
		}

		if (this.edge) {
			this.edge.mesh.instanceMatrix.needsUpdate = true;
			this.edge.depth1Attribute.needsUpdate = true;
			this.edge.depth2Attribute.needsUpdate = true;
		}
	}

	rotate(theta: number) {
		this.wasm?.rotate(theta);
	}
}

function patchedInstancedMesh(
	geometry: THREE.BufferGeometry,
	material: THREE.Material,
	instances: F32ArrSizedPointer
): THREE.InstancedMesh {
	const mesh = new THREE.InstancedMesh(geometry, material, Math.floor(instances.length / 16));
	mesh.instanceMatrix = f32WasmIBA(instances, 16);
	return mesh;
}

function f32WasmIBA(ptr: F32ArrSizedPointer, item_size: number): THREE.InstancedBufferAttribute {
	return new THREE.InstancedBufferAttribute(
		new Float32Array(memory.buffer, ptr.pointer, ptr.length),
		item_size
	);
}
