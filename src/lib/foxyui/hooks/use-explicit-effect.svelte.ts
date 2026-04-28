import { untrack } from 'svelte';

export function useExplicitEffect(
	fn: (fistRun: boolean) => void,
	depFn: () => unknown[],
	blockFirstRun = false
): void {
	let firstRun = true;
	$effect(() => {
		void depFn();
		untrack(() => {
			if (blockFirstRun && firstRun) {
				firstRun = false;
				return;
			}
			fn(firstRun);
			if (firstRun) {
				firstRun = false;
			}
		});
	});
}
