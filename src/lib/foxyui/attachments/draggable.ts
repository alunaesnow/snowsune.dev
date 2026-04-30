export type DragStartData = { x: number; y: number };
export type DragEndData = { x: number; y: number };
export type DragMoveData = { x: number; y: number; dx: number; dy: number };

export type DragCallbacks = {
	start?: (data: DragStartData) => void;
	move?: (data: DragMoveData) => void;
	end?: (data: DragEndData) => void;
};

/**
 * Creates an attachment that runs `fn` when a mousedown event happens outside the
 * element it is attached to.
 *
 * @param callbacks - Functions to run when drag events happen.
 */
export function draggable(callbacks: DragCallbacks): (element: HTMLElement) => () => void {
	return (element: HTMLElement): (() => void) => {
		let x = 0,
			y = 0;

		/** Typeguard to distinguish between mouse and touch events */
		function isTouchEvent(ev: MouseEvent | TouchEvent): ev is TouchEvent {
			return ev.type.startsWith('touch');
		}

		function handleMousedown(event: TouchEvent | MouseEvent) {
			event.preventDefault();
			if (isTouchEvent(event)) {
				x = event.touches[0].clientX;
				y = event.touches[0].clientY;
			} else {
				x = event.clientX;
				y = event.clientY;
			}

			callbacks.start?.({ x, y });

			window.addEventListener('mousemove', handleMousemove, { passive: false });
			window.addEventListener('mouseup', handleMouseup);
			window.addEventListener('touchmove', handleMousemove, { passive: false });
			window.addEventListener('touchend', handleMouseup);
		}

		function handleMousemove(event: TouchEvent | MouseEvent) {
			event.preventDefault();

			let ev: Touch | MouseEvent;
			if (isTouchEvent(event)) {
				ev = event.changedTouches[0];
			} else {
				ev = event;
			}

			const dx = ev.clientX - x;
			const dy = ev.clientY - y;
			x = ev.clientX;
			y = ev.clientY;

			callbacks.move?.({ x, y, dx, dy });
		}

		function handleMouseup(event: TouchEvent | MouseEvent) {
			if (!isTouchEvent(event)) {
				x = event.clientX;
				y = event.clientY;
			}

			callbacks.end?.({ x, y });

			window.removeEventListener('mousemove', handleMousemove);
			window.removeEventListener('mouseup', handleMouseup);
			window.removeEventListener('touchmove', handleMousemove);
			window.removeEventListener('touchend', handleMouseup);
		}

		element.addEventListener('mousedown', handleMousedown, { passive: false });
		element.addEventListener('touchstart', handleMousedown, { passive: false });

		return () => {
			window.removeEventListener('mousemove', handleMousemove);
			window.removeEventListener('mouseup', handleMouseup);
			window.removeEventListener('touchmove', handleMousemove);
			window.removeEventListener('touchend', handleMouseup);

			element.removeEventListener('mousedown', handleMousedown);
			element.removeEventListener('touchstart', handleMousedown);
		};
	};
}
