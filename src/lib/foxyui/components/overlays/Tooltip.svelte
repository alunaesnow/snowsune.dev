<script lang="ts" module>
	import type {
		FlipOptions,
		InlineOptions,
		OffsetOptions,
		Placement,
		ShiftOptions
	} from '@floating-ui/dom';
	import type { Snippet } from 'svelte';

	import { getRandomId } from '$lib/foxyui/utils';

	export type FloaterMiddlewareOptions = {
		offset?: number | OffsetOptions;
		shift?: boolean | ShiftOptions;
		flip?: boolean | FlipOptions;
		inline?: boolean | InlineOptions;
		arrowSize?: number;
		arrowPadding?: number;
		arrowRadius?: number;
	};

	export type TooltipProps = {
		text: string;
		children: Snippet;
		open?: boolean;
		rootElement?: string;
		placement?: Placement;
		middleware?: FloaterMiddlewareOptions;
		correctForIcon?: boolean;
		[key: string]: any;
	};

	function defaultMiddlewareOptions(
		opts: FloaterMiddlewareOptions
	): Required<FloaterMiddlewareOptions> {
		let arrowSize = opts.arrowSize ?? 8;
		return {
			offset: opts.offset ?? 8,
			shift: opts.shift ?? true,
			flip: opts.shift ?? true,
			inline: opts.inline ?? true,
			arrowSize,
			arrowPadding: opts.arrowPadding ?? arrowSize / 2,
			arrowRadius: opts.arrowRadius ?? 2
		};
	}
</script>

<script lang="ts">
	import {
		arrow,
		autoUpdate,
		computePosition,
		flip,
		inline,
		offset,
		shift
	} from '@floating-ui/dom';
	import { fly } from 'svelte/transition';

	let {
		open = false,
		rootElement = 'span',
		placement = 'top',
		middleware = {},
		children,
		text,
		correctForIcon = false,
		...restProps
	}: TooltipProps = $props();

	let rootEl: HTMLElement | null;
	let floaterEl: HTMLElement | null = $state(null);
	let arrowEl: HTMLElement | null = $state(null);
	let floaterId = getRandomId();

	let opts = $derived(defaultMiddlewareOptions(middleware));
	let builtMiddleware = $derived.by(() => {
		const a = [];
		if (opts.offset !== 0) {
			a.push(offset(opts.offset));
		}
		if (opts.shift !== false) {
			a.push(shift(opts.shift === true ? undefined : opts.shift));
		}
		if (opts.flip !== false) {
			a.push(flip(opts.flip === true ? undefined : opts.flip));
		}
		if (opts.inline !== false) {
			a.push(inline(opts.inline === true ? undefined : opts.inline));
		}
		if (arrowEl && opts.arrowSize != 0 && arrow) {
			a.push(arrow({ element: arrowEl, padding: opts.arrowPadding }));
		}
		return a;
	});

	$effect(() => {
		if (floaterEl && rootEl) {
			return autoUpdate(rootEl, floaterEl, updatePosition);
		}
	});

	let arrowSeen = $state(true);
	function updatePosition() {
		if (!(floaterEl && rootEl)) {
			return;
		}
		computePosition(rootEl, floaterEl, {
			placement,
			middleware: builtMiddleware
		}).then(({ x, y, placement, middlewareData }) => {
			if (!(floaterEl && rootEl)) {
				return;
			}
			Object.assign(floaterEl.style, {
				left: `${x}px`,
				top: `${y}px`
			});

			// Accessing the data
			const d = middlewareData.arrow;

			if (arrowEl && d && (d.x != null || d.y != null) && opts.arrowSize) {
				const staticSide =
					{
						top: 'bottom',
						right: 'left',
						bottom: 'top',
						left: 'right'
					}[placement.split('-')[0]] ?? 'bottom';

				const borderSide = {
					top: 'bottom-right',
					right: 'bottom-left',
					bottom: 'top-left',
					left: 'top-right'
				}[placement.split('-')[0]];

				Object.assign(arrowEl.style, {
					left: d.x != null ? `${d.x}px` : '',
					top: d.y != null ? `${d.y}px` : '',
					right: '',
					bottom: '',
					width: `${opts.arrowSize}px`,
					height: `${opts.arrowSize}px`,
					[staticSide]: `-${opts.arrowSize / 2}px`,
					[`border-${borderSide}-radius`]: `${opts.arrowRadius}px`
				});

				if (d.centerOffset) {
					arrowSeen = false;
				} else {
					arrowSeen = true;
				}
			}
		});
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<svelte:element
	this={rootElement}
	onmouseenter={() => (open = true)}
	onmouseleave={() => (open = false)}
	bind:this={rootEl}
	aria-describedby={floaterId}
	{...restProps}
>
	{@render children()}
</svelte:element>

{#if open}
	<div
		transition:fly={{ duration: 300, y: 5 }}
		bind:this={floaterEl}
		id={floaterId}
		role="tooltip"
		class={'absolute top-0 left-0 z-10 w-max max-w-96 rounded-md border bg-gray-800 p-1.5 px-2.5 text-center text-sm text-white shadow-md' +
			(correctForIcon ? ' -translate-x-0.5 -translate-y-0.5' : '')}
	>
		{text}
		<div
			bind:this={arrowEl}
			class:scale-0={!arrowSeen}
			class="absolute -z-10 rotate-45 bg-gray-800 transition-transform duration-200"
		></div>
	</div>
{/if}
