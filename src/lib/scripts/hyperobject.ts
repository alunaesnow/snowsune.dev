import * as THREE from 'three';
import { polygen, type Polytope } from './polygen';
import { HyperObjectWasm } from '$lib/pkg/rs';
import { memory } from '$lib/pkg/rs_bg.wasm';

export class HyperObject {
	poly: Polytope | undefined;
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
		this.poly = polygen(diagram, true);

		const flatVert = this.poly.vertices.flat();
		const flatEdges = this.poly.edges.flat();

		this.wasm = new HyperObjectWasm(new Float32Array(flatVert), new Uint32Array(flatEdges));
		this.wasm.update();

		// TODO: Make these modifiable
		const extraUniforms = {
			nearColor: { value: new THREE.Color().setHSL(60 / 360, 1, 0.5) },
			farColor: { value: new THREE.Color().setHSL(0 / 360, 1, 0.5) },
			depthScaling: { value: 0.2 }
		};

		/// Create vertices
		const vertexGeometry = new THREE.SphereGeometry(0.045, 16, 16);
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

		const vertexDepthAttribute = new THREE.InstancedBufferAttribute(
			new Float32Array(memory.buffer, depths.pointer, depths.length),
			1
		);
		vertexGeometry.setAttribute('depth', vertexDepthAttribute);
		const vertexMesh = new THREE.InstancedMesh(
			vertexGeometry,
			vertexMaterial,
			this.poly.vertices.length
		);
		vertexMesh.instanceMatrix = new THREE.InstancedBufferAttribute(
			new Float32Array(memory.buffer, instances.pointer, instances.length),
			16
		);
		this.scene.add(vertexMesh);
		this.vertex = { mesh: vertexMesh, depthAttribute: vertexDepthAttribute };

		depths.free();
		instances.free();

		// edges
		// const edgeData = getEdgesFromFaces(faces);
		const edgeGeometry = new THREE.CylinderGeometry(0.045, 0.045, 1, 16, 1, true);
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

		const edgeDepth1Attribute = new THREE.InstancedBufferAttribute(
			new Float32Array(memory.buffer, depth1s.pointer, depth1s.length),
			1
		);
		edgeGeometry.setAttribute('depth1', edgeDepth1Attribute);
		const edgeDepth2Attribute = new THREE.InstancedBufferAttribute(
			new Float32Array(memory.buffer, depth2s.pointer, depth2s.length),
			1
		);
		edgeGeometry.setAttribute('depth2', edgeDepth2Attribute);
		const edgeMesh = new THREE.InstancedMesh(edgeGeometry, edgeMaterial, this.poly.edges.length);
		edgeMesh.instanceMatrix = new THREE.InstancedBufferAttribute(
			new Float32Array(memory.buffer, instances2.pointer, instances2.length),
			16
		);
		this.scene.add(edgeMesh);
		this.edge = {
			mesh: vertexMesh,
			depth1Attribute: edgeDepth1Attribute,
			depth2Attribute: edgeDepth2Attribute
		};

		depth1s.free();
		depth2s.free();
		instances2.free();
	}
}
