<script lang="ts" module>
	export type PlaneState = { value: number; locked: boolean };
</script>

<script lang="ts">
	import { Button, Slider } from '$lib/foxyui/components';

	let { plane, state = $bindable() }: { plane: string; state: PlaneState } = $props();
	function onChangeEnd() {
		if (!state.locked) {
			state.value = 0;
		}
	}

	function onToggleLock() {
		state.locked = !state.locked;
		onChangeEnd();
	}

	const colors = {
		X: 'text-red-500',
		Y: 'text-green-500',
		Z: 'text-blue-500',
		W: 'text-cyan-500'
	};

	function color(axis: string) {
		// @ts-expect-error: beans
		return colors[axis];
	}
</script>

<div class="flex w-full items-center space-x-2">
	<span class="font-bold">
		{#each plane.toUpperCase().split('') as char (char)}
			<span class={color(char)}>{char}</span>
		{/each}
	</span>
	<div class="grow">
		<Slider min={-1} max={1} fillFrom="zero" bind:value={state.value} onchangeend={onChangeEnd} />
	</div>
	<Button
		variant="transparent"
		onclick={onToggleLock}
		icon={state.locked ? 'icon-[flowbite--lock-outline]' : 'icon-[flowbite--lock-open-outline]'}
	/>
</div>
