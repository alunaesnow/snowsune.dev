<script lang="ts" module>
	export type PassableInputFrameProps<T> = {
		/** title to show above the input */
		title?: string;
		/** description to show above the input */
		description?: string;
		/** if the input is valid, this prop is bindable */
		invalid?: boolean;
		/** inline inputs dont display titles descriptions or error messages */
		inline?: boolean;
		/** if this input is required to have a value.
		 * enables validation accordingly, if this prop is a
		 * string it will be used as a custom error message */
		required?: string | boolean;
		/** function used to validate input, should return nothing if the
		 * input is valid, or a string describing the error if invalid */
		validator?: (value: T) => string | undefined;
		/** whether to run the validator live.
		 * i.e. each time the value changes,
		 * as opposed to on form submission */
		liveValidate?: boolean;
		/** triggers when the value changes */
		onchange?: (value: T) => void;
		/** triggers when the value changes and is valid,
		 * ONLY IF live validation is enabled */
		onvalidchange?: (value: T) => void;
		/** triggers when the value changes and is invalid,
		 * ONLY IF live validation is enabled */
		oninvalidchange?: (value: T) => void;
	};
</script>

<script lang="ts" generics="T">
	import { type Snippet, onMount, untrack } from 'svelte';

	import { useExplicitEffect } from '$lib/foxyui/hooks';

	import { getFormContext } from './Form.svelte';

	type Props = {
		/** id of the input, used for the form, if one is present */
		id: string;
		/** id of the actual <input> element, used as a target for the label */
		inputId?: string;
		value: T;
		/** auto validator function, will be run along with the passable validator */
		autoValidator?: (value: T) => string | undefined;
		children: Snippet<[{ styleClasses: string }]>;
	} & PassableInputFrameProps<T>;

	let {
		// non-passable props
		id,
		inputId,
		value,
		autoValidator,
		children,
		// passable props
		title,
		description,
		invalid = $bindable(false),
		inline,
		required,
		validator,
		liveValidate,
		onchange,
		onvalidchange,
		oninvalidchange
	}: Props = $props();

	let form = getFormContext();

	let error: string | undefined = $state(undefined);

	let styleClasses = $derived.by(() => {
		let str =
			'inline-flex w-full text-base items-center rounded-lg bg-gray-100 p-2 px-3.5 outline-hidden';
		if (!invalid) {
			str +=
				' text-gray-700 focus-visible:border-primary-500 focus-visible:ring-3 ring-primary-300/90';
		} else {
			str += ' text-red-500 border-red-500 border placeholder:text-red-300 ring-3 ring-red-300/90';
		}
		return str;
	});

	let modifiedValidator = $derived.by(() => {
		if (required || autoValidator) {
			return (val: T) => {
				let validatorResult = validator?.(val);
				if (validatorResult === undefined) {
					let autoValidatorResult = autoValidator?.(val);
					if (autoValidatorResult === undefined) {
						if (required && val === undefined) {
							if (typeof required == 'string') {
								return required;
							} else {
								return 'field cannot be empty';
							}
						}
					}
					return autoValidatorResult;
				}
				return validatorResult;
			};
		}
		return validator;
	});

	useExplicitEffect(
		() => {
			onchange?.(value);
			if (liveValidate) {
				validate(value);
			}
		},
		() => [value, liveValidate],
		true
	);

	function validate(value: T) {
		if (modifiedValidator) {
			error = modifiedValidator(value);
			invalid = error !== undefined;
		}

		if (error === undefined) {
			onvalidchange?.(value);
		} else {
			oninvalidchange?.(value);
		}
	}

	onMount(() => {
		if (form) {
			form.registerInput(id, {
				getValue: function () {
					return value;
				},
				setError: function (message?: string): void {
					error = message;
					invalid = error !== undefined;
				},
				clearError() {
					error = undefined;
					invalid = false;
				},
				selfValidate: modifiedValidator
			});

			return () => {
				form.deregisterInput(id);
			};
		}
	});
</script>

<div>
	{#if title && !inline}
		<div class="mb-1">
			<svelte:element
				this={inputId ? 'label' : 'p'}
				class="text-sm font-medium text-gray-800"
				for={inputId}
			>
				{title}
			</svelte:element>
			{#if description}
				<p class="text-xs text-gray-400">{description}</p>
			{/if}
		</div>
	{/if}
	{@render children({ styleClasses })}
	{#if invalid && error && !inline}
		<p class="mt-1 text-xs text-red-500">{error}</p>
	{/if}
</div>
