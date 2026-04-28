import { on } from 'svelte/events';

import { forceArray } from '$lib/foxyui/utils';

/**
 * Called a function when clicking outside the target elements
 *
 * @example
 * Intended use is with a $effect rune
 * ```ts
 * $effect(() => {
 *   if (elementA && elementB) {
 *     return useClickOutside([elementA, elementB], fn);
 *   }
 * });
 * ```
 *
 * @param node - HTML element or elements to trap the focus on
 */
export function useClickOutside(nodes: HTMLElement | HTMLElement[], fn: () => void): () => void {
	nodes = forceArray(nodes);

	const handleClick = (event: MouseEvent) => {
		const path = event.composedPath();

		if (nodes.every((node) => !path.includes(node))) {
			fn();
		}
	};

	const off = on(document, 'mousedown', handleClick);

	return () => {
		off();
	};
}
