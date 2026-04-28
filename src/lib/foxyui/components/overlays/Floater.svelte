<script lang="ts" module>
	import type {
		FlipOptions,
		InlineOptions,
		Middleware,
		OffsetOptions,
		Placement,
		ShiftOptions
	} from '@floating-ui/dom';
	import type { Snippet } from 'svelte';

	export type MiddlewaresProp = {
		offset?: number | OffsetOptions;
		shift?: boolean | ShiftOptions;
		flip?: boolean | FlipOptions;
		inline?: boolean | InlineOptions;
		arrowSize?: number;
		arrowPadding?: number;
		arrowRadius?: number;
		extras?: Middleware[];
	};

	export type FloaterProps = {
		children: Snippet;
		floater: Snippet;
		open?: boolean;
		disabled?: boolean;
		rootElement?: string;
		placement?: Placement;
		middlewares?: MiddlewaresProp;
		closeOnClickOutside?: boolean;
		focusTrap?: boolean;
		openOnHover?: boolean;
		toggleOnClick?: boolean;
		closeOnEscape?: boolean;
		parentWidth?: boolean;
		onchange?: (open: boolean) => void;
		[key: string]: any;
	};

	function defaultMiddlewareOptions(opts: MiddlewaresProp): Required<MiddlewaresProp> {
		let arrowSize = opts.arrowSize ?? 8;
		return {
			offset: opts.offset ?? 8,
			shift: opts.shift ?? true,
			flip: opts.shift ?? true,
			inline: opts.inline ?? false,
			arrowSize,
			arrowPadding: opts.arrowPadding ?? arrowSize / 2,
			arrowRadius: opts.arrowRadius ?? 2,
			extras: opts.extras ?? []
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

	import { useClickOutside, useFocusTrap } from '$lib/foxyui/hooks';
	import { getRandomId } from '$lib/foxyui/utils';

	let {
		open = $bindable(false),
		disabled = false,
		rootElement = 'div',
		placement = 'bottom',
		middlewares = {},
		closeOnClickOutside = true,
		focusTrap = true,
		openOnHover = false,
		toggleOnClick = false,
		closeOnEscape = true,
		parentWidth = false,
		onchange,
		children,
		floater,
		...restProps
	}: FloaterProps = $props();

	let rootEl: HTMLElement | null;
	let floaterEl: HTMLElement | null = $state(null);
	let arrowEl: HTMLElement | null = $state(null);
	let floaterId = getRandomId();
	let targetId = getRandomId();

	let opts = $derived(defaultMiddlewareOptions(middlewares));
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
		opts.extras.forEach((v) => a.push(v));
		if (arrowEl && opts.arrowSize != 0 && arrow) {
			a.push(arrow({ element: arrowEl, padding: opts.arrowPadding }));
		}
		return a;
	});

	let visible = $derived(open && !disabled);

	$effect(() => {
		if (floaterEl && rootEl) {
			return autoUpdate(rootEl, floaterEl, updatePosition);
		}
	});

	$effect(() => {
		if (floaterEl && rootEl && closeOnClickOutside) {
			return useClickOutside([floaterEl, rootEl], () => (open = false));
		}
	});

	$effect(() => {
		// The extra requirement of visible means the focus trap will be instantly
		// released on close, rather than waiting for the floater element's transition
		// to finish
		if (floaterEl && focusTrap && visible) {
			return useFocusTrap(floaterEl);
		}
	});

	$effect(() => {
		onchange?.(open);
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
				top: `${y}px`,
				width: parentWidth ? `${rootEl.clientWidth}px` : ''
			});

			// Accessing the data
			const d = middlewareData.arrow;

			if (arrowEl && d && (d.x != null || d.y != null) && opts.arrowSize) {
				const side = (placement.split('-')[0] ?? 'top') as 'top' | 'right' | 'bottom' | 'left';

				const staticSide = {
					top: 'bottom',
					right: 'left',
					bottom: 'top',
					left: 'right'
				}[side];

				const tipCorner = {
					top: 'bottom-right',
					right: 'bottom-left',
					bottom: 'top-left',
					left: 'top-right'
				}[side];

				const borderWidth = {
					top: '0px 1px 1px 0px',
					right: '0px 0px 1px 1px',
					bottom: '1px 0px 0px 1px',
					left: '1px 1px 0px 0px'
				}[side];

				Object.assign(arrowEl.style, {
					left: d.x != null ? `${d.x}px` : '',
					top: d.y != null ? `${d.y}px` : '',
					right: '',
					bottom: '',
					width: `${opts.arrowSize}px`,
					height: `${opts.arrowSize}px`,
					[staticSide]: `-${opts.arrowSize / 2}px`,
					[`border-${tipCorner}-radius`]: `${opts.arrowRadius}px`
					// 'border-width': borderWidth
				});

				if (d.centerOffset) {
					arrowSeen = false;
				} else {
					arrowSeen = true;
				}
			}
		});
	}

	function handleKeydownFloater(ev: KeyboardEvent) {
		if (closeOnEscape && ev.key == 'Escape') {
			ev.preventDefault();
			ev.stopPropagation();
			open = false;
		}
	}

	function handleKeydownRoot(ev: KeyboardEvent) {
		if (ev.key == 'Enter' || ev.key == ' ') {
			ev.preventDefault();
			ev.stopPropagation();
			open = !open;
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<svelte:element
	this={rootElement}
	id={targetId}
	onmouseenter={() => openOnHover && (open = true)}
	onmouseleave={() => openOnHover && (open = false)}
	onclick={() => toggleOnClick && !disabled && !disabled && (open = !open)}
	onkeydown={handleKeydownRoot}
	bind:this={rootEl}
	aria-haspopup="dialog"
	aria-expanded={visible}
	aria-controls={floaterId}
	{...restProps}
>
	{@render children()}
</svelte:element>

{#if visible}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		transition:fly={{ duration: 300, y: 5 }}
		bind:this={floaterEl}
		id={floaterId}
		role="dialog"
		tabindex="-1"
		aria-labelledby={targetId}
		onkeydown={handleKeydownFloater}
		class="absolute top-0 left-0 z-45 w-max rounded-lg bg-white shadow-[-10px_-10px_30px_4px_rgba(0,0,0,0.1),10px_10px_30px_4px_rgba(45,78,255,0.15)]"
	>
		{@render floater()}
		<div
			bind:this={arrowEl}
			class:scale-0={!arrowSeen}
			class="absolute -z-10 rotate-45 bg-white transition-transform duration-200"
		></div>
	</div>
{/if}
