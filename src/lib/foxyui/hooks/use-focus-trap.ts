import { on } from 'svelte/events';

/**
 * Traps the focus inside a given element
 *
 * @example
 * Intended use is with a $effect rune
 * ```ts
 * $effect(() => {
 *   if (targetElement && focusTrapActive) {
 *     return useFocusTrap(targetElement);
 *   }
 * });
 * ```
 *
 * @param node - HTML Element to trap the focus on
 * @param returnFocus - Whether to return focus to the last focused element
 * once the focus trap is removed
 */
export function useFocusTrap(node: HTMLElement, returnFocus = true): () => void {
	const previous = document.activeElement as HTMLElement | undefined;

	/** Returns all tabbable elements under the node */
	function focusable() {
		return Array.from(
			node.querySelectorAll(
				'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
			)
		) as HTMLElement[];
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key !== 'Tab') return;

		const current = document.activeElement;

		const elements = focusable() as HTMLElement[];
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
		const autoFocusElement = Array.from(node.querySelectorAll('[data-autofocus]'))[0] as
			| HTMLElement
			| undefined;
		if (autoFocusElement) {
			autoFocusElement.focus({ preventScroll: true });
		} else {
			focusable()[0]?.focus({ preventScroll: true });
		}
	}

	focusInitial();

	const off = on(node, 'keydown', handleKeydown);

	return () => {
		off();

		if (returnFocus) {
			// Return the focus to the original element, ONLY IF the focus
			// is still within the focus trap
			const activeElement = document.activeElement;
			if (!activeElement || node.contains(activeElement)) {
				previous?.focus();
			}
		}
	};
}
