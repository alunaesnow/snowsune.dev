<script lang="ts">
	import { draggable, hotkeys, type DragStartData } from '$lib/foxyui/attachments';
	import { clamp } from '$lib/foxyui/utils';
	import InputFrame, { type PassableInputFrameProps } from './InputFrame.svelte';

	type Props = {
		/** Current value */
		value?: number;
		/** Upper limit. */
		max?: number;
		/** Lower limit. */
		min?: number;
		/** Step size. */
		step?: number;
		/** Where should the track be filled from - "start", "end" or "zero" */
		fillFrom?: 'start' | 'end' | 'zero';
		/** Values at which to display a mark */
		marks?: number[];
	} & PassableInputFrameProps<number>;

	let {
		value = $bindable(0),
		max = 100,
		min = 0,
		step = (max - min) / 100,
		fillFrom = 'start',
		marks = [],
		...inputFrameProps
	}: Props = $props();

	let track: HTMLDivElement | null = $state(null);
	let thumb: HTMLDivElement | null = $state(null);

	function handleDrag(data: DragStartData) {
		if (!track || !thumb) return;

		let { left, right } = track.getBoundingClientRect();

		left += thumb.offsetWidth / 2;
		right -= thumb.offsetWidth / 2;

		const parentWidth = right - left;
		const p = (data.x - left) / parentWidth;
		value = clamp(Math.round((p * (max - min)) / step) * step + min, min, max);
	}

	function handleDragStart(data: DragStartData) {
		thumb?.focus();
		handleDrag(data);
	}

	function getPos(value: number) {
		return `calc(0.5625rem + ${(value - min) / (max - min)} * (100% - 1.125rem))`;
	}
</script>

<InputFrame label {value} {...inputFrameProps}>
	{#snippet children({ invalid: _invalid })}
		<!-- part: root -->
		<div class="h-5 w-full min-w-10">
			<!-- part: track -->
			<div
				class="relative h-2 w-full translate-y-1.5 rounded-full bg-gray-200"
				bind:this={track}
				{@attach draggable({
					start: handleDragStart,
					move: handleDrag
				})}
			>
				<!-- part: thumb -->
				<div
					class="ring-color-100 absolute top-1/2 z-10 h-4.5 w-4.5 -translate-x-1/2 -translate-y-1/2 cursor-pointer rounded-full border-[3px] border-blue-500 bg-white shadow-sm focus-visible:ring focus-visible:outline-none"
					role="slider"
					aria-valuenow={value}
					bind:this={thumb}
					style:left={getPos(value)}
					tabindex="0"
					{@attach hotkeys([
						['ArrowUp,ArrowRight', () => (value = clamp(value + step, min, max))],
						['ArrowDown,ArrowLeft', () => (value = clamp(value - step, min, max))],
						['Home', () => (value = min)],
						['End', () => (value = max)]
					])}
				></div>
				{#if fillFrom === 'start'}
					<div class="h-full rounded-full bg-blue-500" style:width={getPos(value)}></div>
				{:else if fillFrom === 'end'}
					<div
						class="h-full rounded-full bg-blue-500"
						style:width={getPos(min + max - value)}
						style:float="right"
					></div>
				{:else}
					<div
						class="absolute h-full rounded-full bg-blue-500"
						style:left={value < (min + max) / 2 ? getPos(value) : '50%'}
						style:right={value < (min + max) / 2 ? '50%' : getPos(min + max - value)}
					></div>
				{/if}
				<!-- markers -->
				{#each marks as value (value)}
					<div
						class="absolute top-1/2 h-1 w-1 -translate-x-0.5 -translate-y-1/2 rounded-full bg-white"
						style:left={getPos(value)}
					></div>
				{/each}
			</div>
		</div>
	{/snippet}
</InputFrame>
