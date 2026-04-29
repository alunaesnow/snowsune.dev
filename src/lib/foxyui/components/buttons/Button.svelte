<script lang="ts" module>
	import type { Snippet } from 'svelte';
	import type { SvelteHTMLElements } from 'svelte/elements';

	export type ButtonProps<T extends keyof SvelteHTMLElements | undefined> = {
		/** Element to use for the button e.g. `button`, `a` */
		element?: T;
		/** Icon to give the button */
		icon?: string;
		/** Whether to put the icon on the left or right side */
		iconSide?: 'left' | 'right';
		/** Style variant */
		variant?: 'default' | 'filled' | 'light' | 'transparent';
		/** Extra classes to add to the root element */
		rco?: string;
		/** Disabled state, in this state the button cannot be interacted with */
		disabled?: boolean;
		/** On click handler */
		onclick?: () => void;
		/** Button contents */
		children?: Snippet;
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

	let iconOnly = $derived(children === undefined && icon !== undefined);

	let rootClasses = $derived([
		iconOnly ? 'px-2' : 'px-3.5',
		'rounded-md p-2 text-sm transition-colors outline-hidden ring-primary-300/90 focus-visible:ring-3 hover:no-underline inline-flex items-center',
		variant == 'transparent' ? '' : 'font-semibold shadow-xs',
		disabled
			? 'opacity-80 cursor-not-allowed'
			: ['active:translate-y-px cursor-pointer', variant !== 'transparent' && 'hover:shadow-sm'],
		{
			default: ['border bg-white text-gray-800 border-gray-300', !disabled && 'hover:bg-gray-50'],
			filled: ['bg-primary-500 text-white', !disabled && 'hover:bg-primary-600'],
			light: ['text-primary-700 bg-primary-100', !disabled && 'hover:bg-primary-200'],
			transparent: ['font-medium', !disabled && 'hover:bg-gray-100']
		}[variant],
		rco
	]);

	function onClick() {
		if (!disabled) {
			onclick?.();
		}
	}
</script>

<svelte:element
	this={element}
	class={rootClasses}
	tabindex={disabled ? -1 : 0}
	onclick={onClick}
	role="button"
	{...restProps}
>
	{#if children}
		{#if icon && iconSide == 'left'}
			<span class={[icon, 'mr-2 text-base']}></span>
		{/if}
		{@render children()}
		{#if icon && iconSide == 'right'}
			<span class={[icon, 'ml-2 text-base']}></span>
		{/if}
	{:else if icon}
		<span class={[icon, 'text-base']}></span>
	{/if}
</svelte:element>
