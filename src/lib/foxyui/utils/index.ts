const ALPHANUMERIC = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
export function randomString(length = 10): string {
	let str = '';
	for (let i = 0; i < length; i++) {
		str += ALPHANUMERIC.charAt(Math.floor(Math.random() * ALPHANUMERIC.length));
	}
	return str;
}

/** Clamps `num` between `min` and `max` */
export function clamp(num: number, min: number, max: number) {
	return Math.min(Math.max(num, min), max);
}

export function caughtJSONParse(text: string): unknown | null {
	try {
		return JSON.parse(text);
	} catch {
		return null;
	}
}

export function forceArray<T>(arg: T | T[]): T[] {
	if (!Array.isArray(arg)) {
		return [arg];
	}
	return arg;
}

export function mergeVoidFunctions(fns: (() => void) | (() => void)[]): () => void {
	fns = forceArray(fns);
	return () => fns.forEach((fn) => fn());
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function isStringRecord(obj: unknown): obj is Record<string, any> {
	if (obj === null) return false;

	if (typeof obj !== 'object') return false;

	if (Array.isArray(obj)) return false;

	if (Object.getOwnPropertySymbols(obj).length > 0) return false;

	return Object.getOwnPropertyNames(obj).every((prop) => {
		// @ts-expect-error: it can shut up
		return typeof obj[prop] === 'string';
	});
}
