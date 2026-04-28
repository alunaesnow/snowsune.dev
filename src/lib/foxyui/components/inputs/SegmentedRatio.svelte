<script lang="ts" module>
	export type SegmentedRatioEntry = {
		value: string;
		label?: string;
		icon?: string;
	};
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { Tween } from 'svelte/motion';

	import { getRandomId } from '$lib/foxyui/utils';

	import InputFrame, { type PassableInputFrameProps } from './InputFrame.svelte';

	type Props = {
		id?: string;
		entries: SegmentedRatioEntry[];
		value?: string;
	} & PassableInputFrameProps<string>;

	let {
		id = getRandomId(),
		entries,
		value = $bindable(entries[0].value),
		...inputFrameProps
	}: Props = $props();

	let selectedIndex = $derived.by(() => entries.findIndex((entry) => entry.value == value));

	const pillValues = new Tween({ width: 0, height: 0, translateX: 0 }, { duration: 100 });

	let track: HTMLDivElement;
	let buttons: HTMLButtonElement[] = $state([]);
	onMount(() => {
		updatePill(true);
	});

	function updatePill(instant: boolean) {
		pillValues.set(
			{
				height: track.clientHeight,
				width: buttons[selectedIndex].clientWidth,
				translateX: buttons[selectedIndex].offsetLeft + ~~(selectedIndex != 0)
			},
			instant ? { duration: 0 } : undefined
		);
	}
</script>

<InputFrame {id} {value} {...inputFrameProps}>
	{#snippet children({ styleClasses })}
		<div class={styleClasses} style:padding="0.25rem" style:display="inline-block">
			<div class="relative flex" bind:this={track}>
				<div
					style:width={pillValues.current.width + 'px'}
					style:height={pillValues.current.height + 'px'}
					style:transform={`translateX(${pillValues.current.translateX}px)`}
					class=" absolute top-0 left-0 h-5 rounded-md bg-white shadow-none"
				></div>
				{#each entries as entry, i (entry.value)}
					<button
						class={' ring-opacity-90 ring-primary-300 relative z-2 grow rounded-md border-l p-1.5 px-3 text-sm font-medium outline-hidden transition-colors focus-visible:ring ' +
							(selectedIndex == i
								? 'border-gray-100'
								: i > 0 && i != selectedIndex + 1
									? 'border-gray-200 text-gray-500'
									: 'border-gray-100 text-gray-500') +
							(entry.icon ? ' flex items-center justify-center' : '')}
						onclick={(ev) => {
							ev.stopPropagation();
							value = entry.value;
							updatePill(false);
						}}
						bind:this={buttons[i]}
					>
						{#if entry.icon}
							<span class={entry.icon + ' mr-1.5'}></span>
						{/if}
						{entry.label ?? entry.value}
					</button>
				{/each}
			</div>
		</div>
	{/snippet}
</InputFrame>
