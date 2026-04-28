/// Hooks are designed to be used within $effect runes
import { on } from 'svelte/events';

import { localStorageSynced } from './localStorageSynced.svelte';
import { useClickOutside } from './use-click-outside';
import { useExplicitEffect } from './use-explicit-effect.svelte';
import { useFocusTrap } from './use-focus-trap';

export { localStorageSynced };

export { useClickOutside };

export { useFocusTrap };

export { useExplicitEffect };

export type FocusTrapOptions = {
	active?: boolean;
	returnFocus?: 'never' | 'destroy' | 'always';
};

export function withFocusTrap(node: HTMLElement, opts: FocusTrapOptions = {}) {
	const original = document.activeElement as HTMLElement | undefined;
	// let lastActive = original;
	let { active = true, returnFocus = 'always' } = opts;

	let off: (() => void) | undefined;

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

	function dreturnFocus() {
		// Return the focus to the original element, ONLY IF the focus
		// is still within the focus trap
		console.log('returning focus');

		original?.focus();
	}

	if (active) {
		focusInitial();

		off = on(node, 'keydown', handleKeydown);
	}

	return {
		destroy: () => {
			console.log('destroy');
			off?.();
			if (returnFocus != 'never') {
				dreturnFocus();
			}
		},
		update(opts: FocusTrapOptions) {
			if (opts.returnFocus) {
				returnFocus = opts.returnFocus;
			}

			const newActive = opts.active ?? true;
			if (newActive != active) {
				if (!newActive) {
					off?.();
					if (returnFocus == 'always') {
						dreturnFocus();
					}
				} else {
					// lastActive = document.activeElement as HTMLElement | undefined;
					off = on(node, 'keydown', handleKeydown);
				}
				active = newActive;
			}
		}
	};
}
