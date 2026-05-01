<script lang="ts">
	import AnimatedScene from '$lib/components/AnimatedScene.svelte';
	import { Button, SegmentedRadio } from '$lib/foxyui/components';
	import { Polytope } from '$lib/scripts/polytope';
	import { AnimationProfiler } from '$lib/utils';
	import * as THREE from 'three';
	import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
	import type { PlaneState } from './PlaneControl.svelte';
	import PlaneControl from './PlaneControl.svelte';

	let scene: THREE.Scene, camera: THREE.PerspectiveCamera, renderer: THREE.WebGLRenderer;
	let polychoron: Polytope;

	const PLANES = ['xy', 'xz', 'xw', 'yz', 'yw', 'zw'] as const;
	type Plane = (typeof PLANES)[number];
	type PlaneData = {
		axes: [number, number];
		state: PlaneState;
	};

	class Settings {
		planeData: { [key in Plane]: PlaneData };

		constructor() {
			this.planeData = $state({} as { [key in Plane]: PlaneData });
			for (const plane of PLANES) {
				this.planeData[plane] = { axes: planeAxes(plane), state: { value: 0, locked: false } };
			}

			function planeAxes(plane: Plane): [number, number] {
				const map = {
					x: 0,
					y: 1,
					z: 2,
					w: 3
				};
				// @ts-expect-error: stoopid
				return [map[plane[0]], map[plane[1]]];
			}
		}
	}
	const settings = new Settings();

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

		const orbitControls = new OrbitControls(camera, renderer.domElement);
		orbitControls.minDistance = 0.08;

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

		////////// HyperObject //////////

		// polychoron = new Polytope(scene, 'x5x3x3x');
		polychoron = new Polytope(scene, 'x4o3o3o');
		// polychoron = new Polytope(scene, 'x5x3o3o');
	}

	const profiler = new AnimationProfiler({ frames: 1_000 });
	function frame(delta: number) {
		profiler.begin();
		for (const d of Object.values(settings.planeData)) {
			if (Math.abs(d.state.value) > 1e-15) {
				polychoron.rotate(delta * d.state.value * 0.00075, d.axes[0], d.axes[1]);
			}
		}
		profiler.endSection('rotate');
		polychoron.update();
		profiler.endSection('update');
		renderer.render(scene, camera);
		profiler.endSection('render');
	}

	function resize(_canvas: HTMLCanvasElement, width: number, height: number) {
		renderer.setSize(width, height);
		camera.aspect = width / height;
		camera.updateProjectionMatrix();
	}

	function destroy(canvas: HTMLCanvasElement) {}
</script>

<div class="flex h-full w-full p-4 md:space-x-5 md:p-5">
	<div class="relative h-full w-[70%]">
		<div class="h-full w-full overflow-clip rounded-xl bg-white">
			<AnimatedScene callbacks={{ init, frame, resize, destroy }} />
		</div>
		<div class="floaty absolute top-0 left-0 z-20 rounded-br-lg bg-gray-50">
			<button class="px-2 py-1 text-base" aria-label="back">
				<span class="icon-[mingcute--back-2-line]"></span>
			</button>
		</div>
	</div>
	<div class="h-full w-[30%]">
		<p class="ml-2 pt-2 text-xl font-semibold text-gray-600">Polytope Selection</p>
		<div class="mt-3 flex items-center justify-between rounded-lg bg-white p-3.5 px-4">
			<div class="">
				<div class="font-medium">Tesseract</div>
				<div class="text-xs text-gray-500">Coxeter diagram: x4o3o3o</div>
			</div>
			<div class="text-2xl text-gray-700">
				<span class="icon-[lucide--arrow-right-left]"></span>
			</div>
		</div>
		<p class="mt-5 ml-2 text-xl font-semibold text-gray-600">Controls</p>
		<div class="mt-3 w-full space-y-4 rounded-lg bg-white p-3 px-4">
			<div class="grid grid-cols-2 grid-rows-3 gap-3 gap-x-4">
				{#each PLANES as plane (plane)}
					<PlaneControl {plane} bind:state={settings.planeData[plane].state} />
				{/each}
			</div>

			<div class="mx-auto w-fit">
				<Button
					icon="icon-[mingcute--refresh-anticlockwise-1-line]"
					onclick={() => polychoron.resetRotation()}>Reset rotation</Button
				>
			</div>
		</div>
		<div class="mt-3">
			<SegmentedRadio
				entries={[
					{ value: 'rotate', label: 'Rotate', icon: 'icon-[mdi--rotate-orbit]' },
					{ value: 'unfold', label: 'Unfold', icon: 'icon-[tabler--cube-unfolded]' }
				]}
			/>
		</div>
	</div>
</div>

<svelte:head>
	<title>4D Playground · Projection</title>
</svelte:head>

<style>
	.floaty::after {
		width: 0.5rem;
		height: 0.5rem;
		background: transparent;
		bottom: -0.5rem;
		left: 0px;
		position: absolute;
		content: '';
		border-top-left-radius: 100%;
		box-shadow: 0px 0px 0px 50px rgb(249 250 251 / var(--tw-bg-opacity));
		clip: rect(0px, 0.5rem, 0.5rem, 0px);
		display: block;
	}

	.floaty::before {
		width: 0.5rem;
		height: 0.5rem;
		background: transparent;
		top: 0px;
		right: -0.5rem;
		position: absolute;
		content: '';
		border-top-left-radius: 100%;
		box-shadow: 0px 0px 0px 50px rgb(249 250 251 / var(--tw-bg-opacity));
		clip: rect(0px, 0.5rem, 0.5rem, 0px);
		display: block;
	}
</style>
