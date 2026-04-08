// BASED ON: https://stackoverflow.com/questions/19764018/controlling-fps-with-requestanimationframe

export class AnimationLoop {
	#running = false;
	#then = 0;
	#animationFrame = 0;

	constructor(
		public frame: (delta: number) => void,
		public maxFps = Infinity
	) {}

	start() {
		this.#animate(performance.now());
	}

	stop() {
		cancelAnimationFrame(this.#animationFrame);
		this.#running = false;
	}

	get running() {
		return this.#running;
	}

	#animate(now: number) {
		this.#running = true;
		this.#animationFrame = requestAnimationFrame(this.#animate);

		const frameInterval = 1000 / this.maxFps;

		// Calculate the time that passed since the last frame
		const delta = now - this.#then;

		// If enough time has passed, draw the next frame
		if (delta > frameInterval) {
			// Get ready for the next frame by setting then = now, but also adjust for your
			// specified fpsInterval not being a multiple of RAF's interval (16.7ms)
			this.#then = frameInterval > 0 ? now - (delta % frameInterval) : now;

			// Run the frame
			this.frame(delta);
		}
	}
}
