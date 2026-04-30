<script lang="ts" module>
	/* eslint-disable @typescript-eslint/no-explicit-any */
	import { type Component, type ComponentProps, type Snippet, createContext } from 'svelte';

	export type ModalOptions = {
		/** Whether to close the modal when clicking outside of it (defualt true) */
		closeOnClickOutside?: boolean;
		/** Whether to close the modal when the escape key is pressed (default true) */
		closeOnEscape?: boolean;
		/** Whether to trap focus within the modal (recommended) (default true) */
		trapFocus?: boolean;
		/** Id to give the root div of the modal */
		rootId?: string;
		/** If to force opening of the modal - this will close the previous one (default false) */
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
	import { fade, scale } from 'svelte/transition';

	import { randomString } from '$lib/foxyui/utils';
	import { clickOutside, focusTrap, hotkeys } from '$lib/foxyui/attachments';

	let { children }: ModalManagerProps = $props();

	function withDefaultModalOptions(opts: ModalOptions): Required<ModalOptions> {
		return {
			closeOnClickOutside: opts.closeOnClickOutside ?? true,
			closeOnEscape: opts.closeOnEscape ?? true,
			trapFocus: opts.trapFocus ?? true,
			rootId: opts.rootId ?? randomString(),
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
</script>

{#if modal}
	{@const Modal = modal.component}
	<div
		class="fixed top-0 left-0 z-50 grid h-screen w-screen place-items-center bg-gray-900/15 backdrop-blur-xs"
		transition:fade={{ duration: 150 }}
	>
		<div
			transition:scale|global={{ duration: 150, easing: quintInOut }}
			bind:this={modalEl}
			{@attach modal.options.trapFocus && focusTrap(true)}
			{@attach modal.options.closeOnClickOutside && clickOutside(close)}
			{@attach modal.options.closeOnEscape && hotkeys([['Escape', close]])}
		>
			<Modal {...modal.props} />
		</div>
	</div>
{/if}

{@render children()}
