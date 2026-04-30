/**
 * Creates an attachment that calls a callback `fn` when the element is resized.
 * Detection is done using `ResizeObserver`.
 *
 * @param fn - Function to call when the element is resized.
 *
 * @example
 * ```svelte
 * <div {@attach resize(fn)}>
 *   <!-- etc -->
 * </div>
 * ```
 */
export function resize(
	callback: (entries: ResizeObserverEntry[]) => void
): (element: HTMLElement) => () => void {
	return (element: HTMLElement): (() => void) => {
		const ro = new ResizeObserver((entries) => {
			callback(entries);
		});

		ro.observe(element);

		return () => ro.disconnect();
	};
}
