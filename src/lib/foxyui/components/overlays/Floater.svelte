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

	export function buildMiddlewares(
		opts: MiddlewaresProp,
		arrowElement: HTMLElement | null | undefined
	): Middleware[] {
		const a = [] as Middleware[];
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
		opts.extras?.forEach((v) => a.push(v));
		if (arrowElement && opts.arrowSize != 0 && arrow) {
			a.push(arrow({ element: arrowElement, padding: opts.arrowPadding }));
		}
		return a;
	}

	export type FloaterProps = {
		/** Contents of the trigger / root element. */
		children: Snippet;
		/** Floater contents. */
		floater: Snippet;
		/** Whether the floater is open, bindable.*/
		open?: boolean;
		/** A disabled floater cannot be opened (other than programatically), default false. */
		disabled?: boolean;
		/** What type the trigger/root element would be, default "div". */
		rootElement?: string;
		/** Placement of the floater (floating-ui palcements). */
		placement?: Placement;
		/** Middleware options */
		middlewares?: MiddlewaresProp;
		/** Whether to close the floater when a click happens outside of it, default true. */
		closeOnClickOutside?: boolean;
		/** Whether to trap focus within the floater, default true. */
		focusTrap?: boolean;
		/** Whether to open the floater when the trigger element is hovered over, default false. */
		openOnHover?: boolean;
		/** Whether to open/close the floater when the trigger element is clicked, default true. */
		toggleOnClick?: boolean;
		/** Whether to close the floater when the escape key is pressed, and an element of it
		 * is focused, default true. */
		closeOnEscape?: boolean;
		/** Whether to match the width of the trigger (parent) element, default false. */
		parentWidth?: boolean;
		/** Triggers when the state of the floater changes. */
		onchange?: (open: boolean) => void;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
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

	import { clickOutside, focusTrap as focusTrapAttachment, hotkeys } from '$lib/foxyui/attachments';
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
		toggleOnClick = true,
		closeOnEscape = true,
		parentWidth = false,
		onchange,
		children,
		floater,
		...restProps
	}: FloaterProps = $props();

	let rootEl: HTMLElement | null = $state(null);
	let floaterEl: HTMLElement | null = $state(null);
	let arrowEl: HTMLElement | null = $state(null);
	let floaterId = getRandomId();
	let targetId = getRandomId();

	let opts = $derived(defaultMiddlewareOptions(middlewares));
	let builtMiddleware = $derived(buildMiddlewares(opts, arrowEl));

	let visible = $derived(open && !disabled);

	$effect(() => {
		if (floaterEl && rootEl) {
			return autoUpdate(rootEl, floaterEl, updatePosition);
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

				Object.assign(arrowEl.style, {
					left: d.x != null ? `${d.x}px` : '',
					top: d.y != null ? `${d.y}px` : '',
					right: '',
					bottom: '',
					width: `${opts.arrowSize}px`,
					height: `${opts.arrowSize}px`,
					[staticSide]: `-${opts.arrowSize / 2}px`,
					[`border-${tipCorner}-radius`]: `${opts.arrowRadius}px`
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

<svelte:element
	this={rootElement}
	id={targetId}
	onmouseenter={() => openOnHover && !disabled && (open = true)}
	onmouseleave={() => openOnHover && !disabled && (open = false)}
	onclick={() => toggleOnClick && !disabled && (open = !open)}
	bind:this={rootEl}
	aria-haspopup="dialog"
	aria-expanded={visible}
	aria-controls={floaterId}
	{@attach hotkeys([['Enter,Space', () => !disabled && (open = !open)]])}
	{...restProps}
>
	{@render children()}
</svelte:element>

{#if visible}
	<div
		transition:fly={{ duration: 300, y: 5 }}
		bind:this={floaterEl}
		id={floaterId}
		role="dialog"
		tabindex="-1"
		aria-labelledby={targetId}
		class="absolute top-0 left-0 z-45 w-max rounded-lg bg-white shadow-[-10px_-10px_30px_4px_rgba(0,0,0,0.1),10px_10px_30px_4px_rgba(45,78,255,0.15)]"
		// The extra requirement of visible means the focus trap will be instantly
		// released on close, rather than waiting for the floater element's transition
		// to finish
		{@attach focusTrap && visible && focusTrapAttachment(true)}
		{@attach closeOnClickOutside && clickOutside(() => (open = false), rootEl)}
		{@attach closeOnEscape && hotkeys([['Escape', () => (open = false)]])}
	>
		{@render floater()}
		<div
			bind:this={arrowEl}
			class:scale-0={!arrowSeen}
			class="absolute -z-10 rotate-45 bg-white transition-transform duration-200"
		></div>
	</div>
{/if}
