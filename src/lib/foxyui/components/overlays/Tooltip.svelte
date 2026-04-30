<script lang="ts" module>
	import type { Placement } from '@floating-ui/dom';
	import type { Snippet } from 'svelte';

	import { randomString } from '$lib/foxyui/utils';

	export type TooltipProps = {
		/** Text to show on the tooltip */
		text: string;
		/** Contents of the trigger / root element. */
		children: Snippet;
		/** Whether the floater is open, bindable.*/
		open?: boolean;
		/** What type the trigger/root element would be, default "div". */
		rootElement?: string;
		/** Placement of the floater (floating-ui palcements). */
		placement?: Placement;
		/** Middleware options */
		middleware?: MiddlewaresProp;
		/** If the tooltip is displayed for an icon, it may need to be offset slightly to
		 * display at the correct height. Set this prop to true to do that.
		 */
		correctForIcon?: boolean;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		[key: string]: any;
	};

	function defaultMiddlewareOptions(opts: MiddlewaresProp): Required<MiddlewaresProp> {
		let arrowSize = opts.arrowSize ?? 8;
		return {
			offset: opts.offset ?? 8,
			shift: opts.shift ?? true,
			flip: opts.shift ?? true,
			inline: opts.inline ?? true,
			arrowSize,
			arrowPadding: opts.arrowPadding ?? arrowSize / 2,
			arrowRadius: opts.arrowRadius ?? 2,
			extras: []
		};
	}
</script>

<script lang="ts">
	import { autoUpdate, computePosition } from '@floating-ui/dom';
	import { fly } from 'svelte/transition';
	import { type MiddlewaresProp, buildMiddlewares } from './Floater.svelte';

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
	let floaterId = randomString();

	let opts = $derived(defaultMiddlewareOptions(middleware));
	let builtMiddleware = $derived(buildMiddlewares(opts, arrowEl));

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
