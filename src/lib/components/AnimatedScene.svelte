<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	type Props = {
		// assume raw reactivity
		callbacks: {
			init: (canvas: HTMLCanvasElement, width: number, height: number) => void;
			frame: (delta: number) => void;
			resize: (canvas: HTMLCanvasElement, width: number, height: number) => void;
			destroy: (canvas: HTMLCanvasElement) => void;
		};
		maxFps?: number;
		paused?: boolean;
	};
	let { callbacks, maxFps = Infinity, paused = false }: Props = $props();

	type Error = { title: string; description: string; data?: unknown };

	let parent: HTMLDivElement;
	let canvas: HTMLCanvasElement;
	let then = 0;
	let animationFrame = 0;
	let error: Error | undefined = $state();
	let started = $state(false);

	onMount(start);

	function start() {
		try {
			callbacks.init(canvas, parent.clientWidth, parent.clientHeight);
		} catch (e) {
			handleError({
				title: 'Error initializing scene',
				description: 'The scene failed to initialize. Please reload the page to try again.',
				data: e
			});
			return;
		}

		window.addEventListener('resize', handleResize, false);
		canvas.addEventListener('webglcontextlost', handleWebglContextLost, false);
		animate(performance.now());

		started = true;

		return stop;
	}

	function animate(now: number) {
		animationFrame = requestAnimationFrame(animate);

		const frameInterval = 1000 / maxFps;

		// Calculate the time that passed since the last frame
		const delta = now - then;

		// If enough time has passed, draw the next frame
		if (delta > frameInterval) {
			// Get ready for the next frame by setting then = now, but also adjust for your
			// specified fpsInterval not being a multiple of RAF's interval (16.7ms)
			then = frameInterval > 0 ? now - (delta % frameInterval) : now;

			// Run the frame
			try {
				callbacks.frame(delta);
			} catch (e) {
				handleError({
					title: 'Error rendering frame',
					description: 'An error occured during rendering. Please reload the page to try again.',
					data: e
				});
			}
		}
	}

	function stop() {
		cancelAnimationFrame(animationFrame);
		window.removeEventListener('resize', handleResize, false);
		canvas.removeEventListener('webglcontextlost', handleWebglContextLost, false);
		callbacks.destroy(canvas);
	}

	function handleError(e: Error) {
		error = e;
		if (e.data) {
			console.error(e.data);
		}
		stop();
	}

	function handleResize() {
		callbacks.resize(canvas, parent.clientWidth, parent.clientHeight);
	}

	function handleWebglContextLost() {
		handleError({
			title: 'WebGL context lost',
			description: 'Your browser lost the WebGL context. Please reload the page.'
		});
	}
</script>

<div class="relative h-full w-full" bind:this={parent}>
	<canvas class="h-full w-full" bind:this={canvas}></canvas>
	{#if !error && (paused || !started)}
		<div class="absolute top-0 left-0 z-10 h-full w-full bg-gray-900/10" transition:fade>
			<!-- loader from https://codepen.io/supah/pen/BjYLdW -->
			<svg class="spinner" viewBox="0 0 50 50">
				<circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="5" />
			</svg>
		</div>
	{/if}
	{#if error}
		<div class="absolute top-0 left-0 z-10 h-full w-full bg-white">
			<p>{error.title}</p>
			<p>{error.description}</p>
		</div>
	{/if}
</div>

<style>
	.spinner {
		-webkit-animation: rotate 2s linear infinite;
		animation: rotate 2s linear infinite;
		z-index: 2;
		position: absolute;
		top: 50%;
		left: 50%;
		margin: -25px 0 0 -25px;
		width: 50px;
		height: 50px;
	}
	.spinner .path {
		stroke: #ffffff;
		stroke-linecap: round;
		-webkit-animation: dash 1.5s ease-in-out infinite;
		animation: dash 1.5s ease-in-out infinite;
	}

	@-webkit-keyframes rotate {
		100% {
			transform: rotate(360deg);
		}
	}

	@keyframes rotate {
		100% {
			transform: rotate(360deg);
		}
	}
	@-webkit-keyframes dash {
		0% {
			stroke-dasharray: 1, 150;
			stroke-dashoffset: 0;
		}
		50% {
			stroke-dasharray: 90, 150;
			stroke-dashoffset: -35;
		}
		100% {
			stroke-dasharray: 90, 150;
			stroke-dashoffset: -124;
		}
	}
	@keyframes dash {
		0% {
			stroke-dasharray: 1, 150;
			stroke-dashoffset: 0;
		}
		50% {
			stroke-dasharray: 90, 150;
			stroke-dashoffset: -35;
		}
		100% {
			stroke-dasharray: 90, 150;
			stroke-dashoffset: -124;
		}
	}
</style>
