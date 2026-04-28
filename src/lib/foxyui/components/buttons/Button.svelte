<script lang="ts" module>
	import type { Snippet } from 'svelte';
	import type { SvelteHTMLElements } from 'svelte/elements';

	export type ButtonProps<T extends keyof SvelteHTMLElements | undefined> = {
		element?: T;
		icon?: string;
		iconSide?: 'left' | 'right';
		variant?: 'default' | 'filled' | 'light';
		rco?: string;
		disabled?: boolean;
		onclick?: () => void;
		children: Snippet;
	} & (T extends keyof SvelteHTMLElements ? SvelteHTMLElements[T] : SvelteHTMLElements['button']);
</script>

<script lang="ts" generics="T extends keyof SvelteHTMLElements | undefined">
	let {
		element = 'button',
		icon,
		iconSide = 'left',
		variant = 'default',
		rco = '',
		disabled = false,
		onclick,
		children,
		...restProps
	}: ButtonProps<T> = $props();

	let rootClasses = $derived.by(() => {
		let c =
			'rounded-md p-2 px-3.5 text-sm font-semibold shadow-xs transition-colors outline-hidden ring-varcolor-300/90 focus-visible:ring hover:shadow-sm inline-flex items-center cursor-pointer ';

		if (!disabled) {
			c += 'active:translate-y-px ';
		}

		switch (variant) {
			case 'default':
				c += 'border bg-white text-gray-800 border-gray-300';
				break;
			case 'filled':
				if (!disabled) {
					c += 'bg-var-500 text-white hover:bg-var-600';
				} else {
					c += 'bg-var-500 text-white opacity-75';
				}

				break;
			case 'light':
				c += 'text-var-700 bg-var-100';
				break;
		}

		return c + ' ' + rco;
	});

	function onClick() {
		if (!disabled) {
			onclick?.();
		}
	}
</script>

<svelte:element
	this={element}
	class={rootClasses}
	tabindex="0"
	onclick={onClick}
	role="button"
	{...restProps}
>
	{#if icon && iconSide == 'left'}
		<span class={icon + ' mr-2 text-base'}></span>
	{/if}
	{@render children()}
	{#if icon && iconSide == 'right'}
		<span class={icon + ' ml-2 text-base'}></span>
	{/if}
</svelte:element>
