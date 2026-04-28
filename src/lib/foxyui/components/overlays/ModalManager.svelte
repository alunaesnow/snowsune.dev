<script lang="ts" module>
	import { type Component, type ComponentProps, type Snippet, createContext } from 'svelte';

	export type ModalOptions = {
		/** whether to close the modal when clicking outside of it (defualt true) */
		closeOnClickOutside?: boolean;
		/** whether to close the modal when the escape key is pressed (default true) */
		closeOnEscape?: boolean;
		/** whether to trap focus within the modal (recommended) (default true) */
		trapFocus?: boolean;
		/** id to give the root div of the modal */
		rootId?: string;
		/** if to force opening of the modal, this will close the previous one (default false) */
		force?: boolean;
	};

	export type ModalsContext = {
		/**
		 * Open a new modal, will throw an error if one is already open!
		 * returns a promise which resolves with data from the modals submit function,
		 * or with `null` if the modal is closed for any other reason
		 */
		open: <T extends Component<any, any, string>>(
			component: T,
			props: Omit<ComponentProps<T>, 'close' | 'submit'>,
			options?: ModalOptions
		) => Promise<
			ComponentProps<T> extends {
				submit: (data: infer Data) => void;
			}
				? Data | null
				: null
		>;
		/** Close the open modal. returns bool representing if a modal was open */
		close(): boolean;
	};

	const [getModalsContext, setModalsContext] = createContext<ModalsContext>();
	export { getModalsContext };

	export type ModalManagerProps = {
		children: Snippet;
	};
</script>

<script lang="ts">
	import { quintInOut } from 'svelte/easing';
	import { on } from 'svelte/events';
	import { fade, scale } from 'svelte/transition';

	import { useClickOutside, useFocusTrap } from '$lib/foxyui/hooks';
	import { getRandomId } from '$lib/foxyui/utils';

	let { children }: ModalManagerProps = $props();

	function withDefaultModalOptions(opts: ModalOptions): Required<ModalOptions> {
		return {
			closeOnClickOutside: opts.closeOnClickOutside ?? true,
			closeOnEscape: opts.closeOnEscape ?? true,
			trapFocus: opts.trapFocus ?? true,
			rootId: opts.rootId ?? getRandomId(),
			force: opts.force ?? false
		};
	}

	type Modal = {
		component: Component;
		props: Record<any, unknown>;
		options: Required<ModalOptions>;
		resolve: (value: any | null) => void;
	};

	let modal: Modal | undefined = $state(undefined);
	let modalEl: HTMLDivElement | null = $state(null);

	function close(): boolean {
		if (modal) {
			modal?.resolve(null);
			modal = undefined;
			return true;
		}
		return false;
	}

	setModalsContext({
		open(component, props, options = {}) {
			if (modal) {
				if (options.force === true) {
					close();
				} else {
					throw new Error('tried to open modal, but one is already open!');
				}
			}
			return new Promise((resolve, _reject) => {
				modal = {
					component,
					props: {
						...props,
						close,
						submit: (val: any) => {
							resolve(val);
							modal = undefined;
						}
					},
					options: withDefaultModalOptions(options),
					resolve
				};
			});
		},
		close
	});

	// Close on escape
	$effect(() => {
		if (modalEl && modal?.options.closeOnEscape) {
			modalEl.focus();
			return on(modalEl, 'keydown', (ev: KeyboardEvent) => {
				if (ev.key == 'Escape') {
					ev.preventDefault();
					close();
				}
			});
		}
	});

	// Close on click outside
	$effect(() => {
		if (modalEl && modal?.options.closeOnClickOutside) {
			return useClickOutside(modalEl, close);
		}
	});

	// Focus trap
	$effect(() => {
		if (modalEl && modal?.options.closeOnClickOutside) {
			return useFocusTrap(modalEl, true);
		}
	});
</script>

{#if modal}
	{@const Component = modal.component}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="fixed top-0 left-0 z-50 grid h-screen w-screen place-items-center bg-gray-900/15 backdrop-blur-xs"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="relative overflow-hidden rounded-xl bg-white shadow-lg"
			transition:scale|global={{ duration: 150, easing: quintInOut }}
			bind:this={modalEl}
		>
			<Component {...modal.props} />
		</div>
	</div>
{/if}

{@render children()}
