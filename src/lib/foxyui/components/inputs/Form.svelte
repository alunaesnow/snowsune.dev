<script lang="ts" module>
	type InputFunctions = {
		getValue(): any;
		setError(message?: string): void;
		clearError(): void;
		selfValidate?(value: unknown): string | undefined;
	};

	export type FormContext = {
		registerInput(id: string, functions: InputFunctions): void;
		deregisterInput(id: string): void;
	};

	export function getFormContext() {
		return getContext<FormContext | undefined>('form');
	}

	export type FormValidator<Schema> = (
		data: Schema
	) => { [id in keyof Schema]: string | undefined } | undefined;

	export type FormErrors<Schema> = Partial<{ [id in keyof Schema]: string }>;
</script>

<script lang="ts" generics="Schema">
	import { type Snippet, getContext, setContext } from 'svelte';

	type Props = {
		oncomplete: (data: Schema) => void;
		onerror?: (errors: FormErrors<Schema>) => void;
		validator?: FormValidator<Schema>;
		children: Snippet<[{ submit: () => void }]>;
	};
	let { oncomplete, onerror, validator, children }: Props = $props();

	let inputs: Record<string, InputFunctions> = {};

	setContext<FormContext>('form', {
		registerInput(id, functions) {
			if (id in inputs) {
				console.warn(`re-registering input with id ${id}`);
			}
			inputs[id] = functions;
		},
		deregisterInput(id) {
			delete inputs[id];
		}
	});

	function submit() {
		let data: Record<string, unknown> = {};
		let errors: Record<string, string> = {};
		for (const [id, functions] of Object.entries(inputs)) {
			data[id] = functions.getValue();
			if (functions.selfValidate) {
				let v = functions.selfValidate(data[id]);
				if (v !== undefined) {
					errors[id] = v;
				}
			}
		}
		if (validator) {
			let validatorReturn = validator(data as Schema);
			if (validatorReturn) {
				for (const id in validatorReturn) {
					if (validatorReturn[id] !== undefined) {
						errors[id] = validatorReturn[id];
					}
				}
			}
		}

		for (const id of Object.keys(inputs)) {
			inputs[id].setError(errors[id]);
		}

		if (Object.keys(errors).length > 0) {
			onerror?.(errors as FormErrors<Schema>);
		} else {
			oncomplete?.(data as Schema);
		}
	}
</script>

{@render children({ submit })}
