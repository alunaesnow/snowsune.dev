/* eslint-disable @typescript-eslint/no-explicit-any */
import { caughtJSONParse } from '$lib/foxyui/utils';

/**
 * Creates a state synchronised to local storage
 *
 * @remarks
 * This state will not be synchronised across other iterations, it is advised to have a single
 * copy shared between components if that is needed
 *
 * @param key - The local storage key to use when reading/writing
 *
 * @param initial - Initial/default value for the state
 *
 * @param io - If provided `io.parse` will be used to parse the data read from localstorage. parse
 * should return a value or `null` if the data is invalid. `io.deposit` will be used to convert
 * the state into a JSON stringifiable value before saving to localStorage.
 */
export function localStorageSynced<T>(
	key: string,
	initial: T,
	io?: { parse(data: unknown): T | null; deposit(data: T): any }
): T {
	// Read from local storage
	let data: T;
	if (typeof localStorage !== 'undefined') {
		const existingData = localStorageGetJSON<T>(key, io?.parse);
		data = existingData === null ? initial : existingData;
	} else {
		data = initial;
	}

	const state = $state<T>(data);

	$effect(() => {
		try {
			localStorage.setItem(key, JSON.stringify(io?.deposit ? io.deposit(state) : state));
		} catch (e) {
			console.warn('Failed to sync state to localStorage', { error: e, state });
		}
	});

	return state;
}

function localStorageGetJSON<T>(key: string, parser?: (data: unknown) => T | null): T | null {
	const data = localStorage.getItem(key);
	if (data) {
		const j = caughtJSONParse(data);
		if (j) {
			if (!parser) {
				return j as T;
			}

			return parser(j);
		}
	}
	return null;
}
