import { on } from 'svelte/events';

/**
 * Creates an attachment that runs `fn` when a mousedown event happens outside the
 * element it is attached to.
 *
 * @param fn - Will be run whenever a mousedown event happens outside the target
 * element's node tree.
 *
 * @param extraElements - Extra elements that must be clicked outside.
 *
 * ## Examples
 * @example
 * ```svelte
 * <div {@attach clickOutside(() => console.log("clicked outside the div"))}>
 *   <!-- etc -->
 * </div>
 * ```
 */
export function clickOutside(
	fn: () => void,
	...extraElements: (HTMLElement | null)[]
): (element: HTMLElement) => () => void {
	return (element: HTMLElement): (() => void) => {
		const handleClick = (event: MouseEvent) => {
			const path = event.composedPath();

			if (!path.includes(element) && !extraElements.some((el) => el && path.includes(el))) {
				fn();
			}
		};

		const off = on(document, 'mousedown', handleClick);

		return () => {
			off();
		};
	};
}
