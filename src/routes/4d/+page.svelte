<script lang="ts">
	import AnimatedScene from '$lib/components/AnimatedScene.svelte';
	import { HyperObject } from '$lib/scripts/hyperobject';
	import * as THREE from 'three';
	import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

	let scene: THREE.Scene, camera: THREE.PerspectiveCamera, renderer: THREE.WebGLRenderer;
	let cube: THREE.Mesh;
	let hyperobject: HyperObject;

	function init(canvas: HTMLCanvasElement, width: number, height: number) {
		// ////////// Inital Setup //////////
		scene = new THREE.Scene();
		camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
		renderer = new THREE.WebGLRenderer({
			canvas,
			antialias: true,
			alpha: true
		});
		renderer.sortObjects = true;

		scene.background = new THREE.Color(0xffffff);
		renderer.setSize(width, height);

		new OrbitControls(camera, renderer.domElement);

		////////// Scene Setup //////////
		// Camera
		camera.position.set(0, 1.1, -2.75);
		camera.lookAt(new THREE.Vector3(0, 0, 0));

		// Lighting
		const ambientLight = new THREE.AmbientLight(0xd1d1d1, 0.75 * Math.PI);
		scene.add(ambientLight);

		const directionalLight = new THREE.PointLight(0xffffff, 0.75 * Math.PI, 0, 0);
		directionalLight.position.y = 10;
		scene.add(directionalLight);

		// Directional Light
		// const directionalLight = new THREE.DirectionalLight(0xffffff, 5);
		// directionalLight.position.x = 0;
		// directionalLight.position.y = 5;
		// directionalLight.position.z = 5;
		// scene.add(directionalLight);

		const material = new THREE.MeshPhongMaterial({
			color: new THREE.Color().setHSL(30 / 360, 1, 0.5),
			shininess: 100
		});
		const geometry = new THREE.CapsuleGeometry(0.2, 1, 100, 100);
		cube = new THREE.Mesh(geometry, material);

		cube.rotation.x = 22.5;
		cube.rotation.y = 45;

		// scene.add(cube);

		////////// HyperObject //////////

		hyperobject = new HyperObject(scene);
		hyperobject.loadPoly('x x x x');
	}

	function frame(delta: number) {
		renderer.render(scene, camera);

		// let time = performance.now();
		// cube.rotation.x = time / 2000;
		// cube.rotation.y = time / 1000;
	}

	function resize(_canvas: HTMLCanvasElement, width: number, height: number) {
		renderer.setSize(width, height);
		camera.aspect = width / height;
		camera.updateProjectionMatrix();
	}

	function destroy(canvas: HTMLCanvasElement) {}
</script>

<AnimatedScene callbacks={{ init, frame, resize, destroy }} />
