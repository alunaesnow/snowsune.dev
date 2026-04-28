<script lang="ts">
	import { getRandomId } from '$lib/foxyui/utils';

	import InputFrame, { type PassableInputFrameProps } from './InputFrame.svelte';

	type Props = {
		id?: string;
		value?: number;
		max?: number;
		min?: number;
		step?: number;
		placeholder?: string;
		rco?: string;
		hidearrows?: boolean;
		autosize?: boolean;
	} & PassableInputFrameProps<number | undefined>;

	let {
		id = getRandomId(),
		value = $bindable(undefined),
		placeholder,
		max,
		min,
		step = 1,
		rco,
		hidearrows = false,
		autosize = false,
		...inputFrameProps
	}: Props = $props();

	let ch = $state(1);

	$effect(() => {
		if (autosize) {
			ch = value?.toString().length ?? 1;
		}
	});

	let autoValidator = $derived.by(() => {
		if (min || max) {
			return (val: number | undefined) => {
				if (val !== undefined) {
					if (min !== undefined && val < min) {
						return `value must be ${min} or more`;
					}
					if (max !== undefined && val > max) {
						return `value must be ${max} or less`;
					}
				}
				return undefined;
			};
		}
		return undefined;
	});

	function increment() {
		if (max === undefined || (value ?? 0) < max) {
			value = (value ?? (min ?? 0) - step) + step;
		}

		fixBounds();
	}

	function decrement() {
		if (min === undefined || (value ?? 0) > min) {
			value = (value ?? (max ?? 0) + step) - step;
		}

		fixBounds();
	}

	function fixBounds() {
		if (value !== undefined) {
			if (max !== undefined && value > max) {
				value = max;
			}

			if (min != undefined && value < min) {
				value = min;
			}
		}
	}

	function onchange(ev: any) {
		let valueStr: string = ev.target.value;
		let num = parseInt(valueStr);
		if (Number.isNaN(num)) {
			value = undefined;
		} else {
			value = num;
		}
	}
</script>

<InputFrame {id} inputId={id} {value} {autoValidator} {...inputFrameProps}>
	{#snippet children({ styleClasses })}
		<div class={'relative inline-block w-full text-sm ' + (rco ?? '')}>
			<input
				type="number"
				{id}
				{max}
				{min}
				{step}
				{value}
				{placeholder}
				{onchange}
				style:width={autosize ? `calc(1.8rem + ${ch}ch)` : undefined}
				onfocus={autosize
					? () => (ch = Math.max(value ?? 0, max ?? 0).toString().length + 1)
					: undefined}
				onfocusout={autosize ? () => (ch = value?.toString().length ?? 1) : undefined}
				class={['number-input', styleClasses, { 'text-center': autosize }]}
			/>
			{#if !hidearrows}
				<div class="absolute top-0 right-0 grid h-full w-8 grid-cols-1 grid-rows-2">
					<button
						tabindex="-1"
						aria-label="increment"
						onclick={increment}
						class="rounded-tr-lg rounded-bl hover:bg-white"
					>
						<span class="i-[mingcute--up-line] translate-y-0.5"></span>
					</button>
					<button
						tabindex="-1"
						aria-label="decrement"
						onclick={decrement}
						class="rounded-tl rounded-br-lg hover:bg-white"
					>
						<span class="i-[mingcute--down-line] translate-y-px"></span>
					</button>
				</div>
			{/if}
		</div>
	{/snippet}
</InputFrame>

<style>
	.number-input::-webkit-inner-spin-button,
	.number-input::-webkit-outer-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	.number-input {
		appearance: textfield;
	}
</style>
