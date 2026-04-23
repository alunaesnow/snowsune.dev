/** A `requestAnimationFrame` loop with a fps limit.
 * BASED ON: https://stackoverflow.com/questions/19764018/controlling-fps-with-requestanimationframe
 */
export class AnimationLoop {
	#running = false;
	#then = 0;
	#animationFrame = 0;

	/** `frame` is the code triggered every animation frame, `maxFps` if set will limit frame to be
	 * called at most `maxFps` times per second.
	 */
	constructor(
		public frame: (delta: number) => void,
		public maxFps = Infinity
	) {}

	/** Start the `requestAnimationFrame` loop. */
	start() {
		this.#animate(performance.now());
	}

	/** Stop the `requestAnimationFrame` loop. */
	stop() {
		cancelAnimationFrame(this.#animationFrame);
		this.#running = false;
	}

	/** True if the animation loop is running. */
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

/** Class used to profile a section of code, logging agregate performance statistics to the console.
 * Intended for use in profiling animations.
 */
export class AnimationProfiler {
	sectionTimes: Record<string, number> = {};
	totalTime = 0;
	frameCount = 0;
	sectionStart = 0;

	/** Create a new AnimationProfiler with the logging interval set to ever `frames` frames or
	 * every `duration` seconds.
	 */
	constructor(public logInterval: { frames: number } | { duration: number }) {}

	/** Mark the start of the code you are profiling. If the logging interval has been reached
	 * will also log performance statistics in the console.
	 */
	begin() {
		this.frameCount++;
		if (
			('frames' in this.logInterval && this.frameCount == this.logInterval.frames) ||
			('duration' in this.logInterval && this.totalTime >= this.logInterval.duration)
		) {
			console.log(
				`${this.frameCount} frames in ${this.totalTime} ms (${(this.totalTime / this.frameCount).toFixed(3)} ms/frame)`
			);
			Object.entries(this.sectionTimes).forEach(([name, time]) => {
				console.log(`> ${name}: ${time} ms total, ${(time / this.frameCount).toFixed(3)}ms /frame`);
			});
			this.sectionTimes = {};
			this.frameCount = 0;
			this.totalTime = 0;
		}
		this.sectionStart = performance.now();
	}

	/** Mark the end of a section of profiled code with id `name`. */
	endSection(name: string) {
		const end = performance.now();
		this.sectionTimes[name] = (this.sectionTimes[name] ?? 0) + end - this.sectionStart;
		this.totalTime += end - this.sectionStart;
		this.sectionStart = end;
	}

	/** Set the logging interval to every `frames` frames or `duration` seconds. */
	setInterval(logInterval: { frames: number } | { duration: number }) {
		this.logInterval = logInterval;
	}
}
