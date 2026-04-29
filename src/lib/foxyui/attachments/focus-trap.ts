import { on } from 'svelte/events';

/** Returns all tabbable elements under `element`. */
function focusable(element: HTMLElement): HTMLElement[] {
	return Array.from(
		element.querySelectorAll(
			'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
		)
	);
}

/**
 * Creates an attachment that traps the focus within the element it is attached to.
 *
 * @param returnFocus - Whether to return focus to the last focused element
 * once the focus trap is removed.
 *
 * ## Auto focus
 * If you would like to set an element as the initial element to be focused, give it the
 * attribute `data-autofocus`. Otherwise the first tabbable element will be autofocused.
 *
 * ## Examples
 * @example
 * ```svelte
 * <div {@attach focusTrap(true)}>
 *   <button>button 1</button>
 *   <!-- data-autofocus is optional! -->
 *   <button data-autofocus>button 2</button>
 *   <button>button 3</button>
 * </div>
 * ```
 */
export function focusTrap(returnFocus = true): (element: HTMLElement) => () => void {
	return (element: HTMLElement): (() => void) => {
		const previous = document.activeElement as HTMLElement | undefined;

		function handleKeydown(event: KeyboardEvent) {
			if (event.key !== 'Tab') return;

			const current = document.activeElement;

			const elements = focusable(element) as HTMLElement[];
			const first = elements.at(0);
			const last = elements.at(-1);

			if (event.shiftKey && current === first) {
				last?.focus();
				event.preventDefault();
				event.stopPropagation();
			}

			if (!event.shiftKey && current === last) {
				first?.focus();
				event.preventDefault();
				event.stopPropagation();
			}
		}

		/** Focus the first element with data-autofocus attribute or otherwise the first focusable element */
		function focusInitial() {
			const autoFocusElement = Array.from(element.querySelectorAll('[data-autofocus]'))[0] as
				| HTMLElement
				| undefined;
			if (autoFocusElement) {
				autoFocusElement.focus({ preventScroll: true });
			} else {
				focusable(element)[0]?.focus({ preventScroll: true });
			}
		}

		focusInitial();

		const off = on(element, 'keydown', handleKeydown);

		return () => {
			off();

			if (returnFocus) {
				// Return the focus to the original element, ONLY IF the focus
				// is still within the focus trap
				const activeElement = document.activeElement;
				if (!activeElement || element.contains(activeElement)) {
					previous?.focus();
				}
			}
		};
	};
}
