// Code below is based on:
// https://github.com/mantinedev/mantine/blob/master/src/mantine-hooks/src/use-hotkeys/parse-hotkey.ts

import { on } from 'svelte/events';

export type HotkeyOptions = {
	/** Defaults to `true` */
	preventDefault: boolean;
	/** Defaults to `false` */
	stopPropogation: boolean;
};
export type HotkeySet = [string, (event: KeyboardEvent) => void, Partial<HotkeyOptions>?];

/**
 * Creates an attachment which listens for key combinations on the attached node,
 * executing callbacks in response.
 *
 * @param hotkeySets Define the hotkeys here (see the example).
 *
 * The hotkey string has the following rules:
 * - `ctrl+shift+X` – include modifiers with +; supported modifiers are: alt, ctrl, meta, shift, mod
 * - `ArrowLeft,ArrowRight` – use commas to specify multiple keybinds
 * - `mod+S` – detects ⌘+S on macOS and Ctrl+S on Windows and Linux
 * - `alt + shift + L` – whitespace is allowed
 * - For the space key use `Space`
 *
 * @param gloablOptions See type HotkeyOptions.
 *
 * @example
 * ```svelte
 * <button
 *   {@attach hotkeys([
 *     ['ArrowUp,ArrowRight', () => increment()],
 *     ['ArrowDown,ArrowLeft', () => decrement()],
 *     ['ctrl+shift+r', () => reset()],
 *     ['ctrl+a', () => alert('ctrl+a'), { preventDefault: false }],
 *   ])}
 * />
 * ```
 */
export function hotkeys(
	hotkeySets: HotkeySet[],
	gloablOptions?: Partial<HotkeyOptions>
): (element: HTMLElement) => () => void {
	type HotkeyGroup = {
		matcher: (event: KeyboardEvent) => boolean;
		callback: () => void;
		options: HotkeyOptions;
	};

	const groups: HotkeyGroup[] = hotkeySets.map(([hotkeysString, callback, localOptions]) => {
		const options: HotkeyOptions = {
			preventDefault: true,
			stopPropogation: false
		};

		Object.assign(options, gloablOptions, localOptions);

		return {
			matcher: getHotkeyMatcher(hotkeysString),
			callback,
			options
		} as HotkeyGroup;
	});

	const handler = function (event: KeyboardEvent) {
		groups.forEach((group) => {
			if (group.matcher(event)) {
				if (group.options.preventDefault) {
					event.preventDefault();
				}
				if (group.options.stopPropogation) {
					event.stopPropagation();
				}
				group.callback();
			}
		});
	};

	return (element: HTMLElement): (() => void) => {
		const off = on(element, 'keydown', handler);

		return () => {
			off();
		};
	};
}

type KeyboardModifiers = {
	alt: boolean;
	ctrl: boolean;
	meta: boolean;
	mod: boolean;
	shift: boolean;
};

type Hotkey = KeyboardModifiers & {
	key?: string;
};

function parseHotkeysString(hotkeys: string): Hotkey[] {
	return hotkeys.split(',').map((hotkey) => {
		const keys = hotkey
			.toLowerCase()
			.split('+')
			.map((part) => part.trim());

		const modifiers: KeyboardModifiers = {
			alt: keys.includes('alt'),
			ctrl: keys.includes('ctrl'),
			meta: keys.includes('meta'),
			mod: keys.includes('mod'),
			shift: keys.includes('shift')
		};

		const reservedKeys = ['alt', 'ctrl', 'meta', 'shift', 'mod'];

		const freeKey = keys.find((key) => !reservedKeys.includes(key));

		return {
			...modifiers,
			key: freeKey
		};
	});
}

function isExactHotkey(hotkey: Hotkey, event: KeyboardEvent): boolean {
	const { alt, ctrl, meta, mod, shift, key } = hotkey;
	const { altKey, ctrlKey, metaKey, shiftKey, key: pressedKey } = event;

	if (alt !== altKey) {
		return false;
	}
	if (mod) {
		if (!ctrlKey && !metaKey) {
			return false;
		}
	} else {
		if (ctrl !== ctrlKey) {
			return false;
		}
		if (meta !== metaKey) {
			return false;
		}
	}
	if (shift !== shiftKey) {
		return false;
	}

	if (
		key &&
		(pressedKey.toLowerCase() === key.toLowerCase() ||
			event.code.replace('Key', '').toLowerCase() === key.toLowerCase())
	) {
		return true;
	}

	return false;
}

export function getHotkeyMatcher(hotkeysString: string) {
	const hotkeys = parseHotkeysString(hotkeysString);
	return (event: KeyboardEvent) => hotkeys.some((hotkey) => isExactHotkey(hotkey, event));
}
