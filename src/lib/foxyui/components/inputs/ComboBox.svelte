<script lang="ts">
	import Floater from '../overlays/Floater.svelte';
	import InputFrame, { type PassableInputFrameProps } from './InputFrame.svelte';

	type Props = {
		values: readonly string[];
		value?: string;
		placeholderText?: string;
	} & PassableInputFrameProps<string | undefined>;

	let {
		values,
		value = $bindable(undefined),
		placeholderText = 'no selection',
		...inputFrameProps
	}: Props = $props();

	let open = $state(false);
	function select(selected: string) {
		value = selected;
		open = false;
	}
</script>

<InputFrame {value} {...inputFrameProps}>
	{#snippet children({ styleClasses })}
		<Floater bind:open toggleOnClick parentWidth middlewares={{ arrowSize: 0 }}>
			<button class={styleClasses}>
				<p>
					{#if value === undefined}
						{placeholderText}
					{:else}
						{value}
					{/if}
				</p>
				<div class="grow"></div>
				<span class="-mr-2 ml-4 icon-[heroicons-outline--selector] text-xl text-gray-400"></span>
			</button>

			{#snippet floater()}
				<div class="text-gray-700">
					{#each values as value (value)}
						<button class="w-full px-3.5 py-2 hover:bg-gray-50" onclick={() => select(value)}>
							{value}
						</button>
					{/each}
				</div>
			{/snippet}
		</Floater>
	{/snippet}
</InputFrame>
