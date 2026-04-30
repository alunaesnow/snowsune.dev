<script lang="ts" module>
	export type PassableInputFrameProps<T> = {
		/** Id of the input, used for the form if one is present. If left undefined
		 * a random id will be generated.
		 */
		id?: string;
		/** Title to show above the input. */
		title?: string;
		/** Description to show above the input. */
		description?: string;
		/** If the input is valid, this prop is bindable. */
		invalid?: boolean;
		/** Inline inputs dont display titles descriptions or error messages. */
		inline?: boolean;
		/** If this input is required to have a value.
		 * enables validation accordingly, if this prop is a
		 * string it will be used as a custom error message. */
		required?: string | boolean;
		/** Function used to validate input, should return nothing if the
		 * input is valid, or a string describing the error if invalid. */
		validator?: (value: T) => string | undefined;
		/** Whether to run the validator live.
		 * i.e. each time the value changes,
		 * as opposed to on form submission. */
		liveValidate?: boolean;
		/** Triggers when the value changes, regardless of validity. */
		onchange?: (value: T) => void;
		/** Triggers when the value changes and is valid,
		 * ONLY IF live validation is enabled. */
		onvalidchange?: (value: T) => void;
		/** Triggers when the value changes and is invalid,
		 * ONLY IF live validation is enabled. */
		oninvalidchange?: (value: T) => void;
	};
</script>

<script lang="ts" generics="T">
	import { type Snippet, onMount } from 'svelte';

	import { useExplicitEffect } from '$lib/foxyui/hooks';

	import { getFormContext } from './Form.svelte';
	import { randomString } from '$lib/foxyui/utils';

	type Props = {
		value: T;
		children: Snippet<[{ styleClasses: string[]; inputId: string; invalid: boolean }]>;
		/** If to use a <label> element for the title. Defaults to false. */
		label?: boolean;
		/** Auto validator function, will be run along with the passable validator. */
		autoValidator?: (value: T) => string | undefined;
	} & PassableInputFrameProps<T>;

	let {
		// non-passable props
		value,
		children,
		id = randomString(8),
		label = false,
		autoValidator,
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

	let inputId = $derived(`${id}-${randomString(5)}`);

	let form = getFormContext();

	let error: string | undefined = $state(undefined);

	let styleClasses = $derived([
		'inline-flex w-full text-base items-center rounded-lg bg-gray-100 p-2 px-3.5 outline-hidden',
		invalid
			? 'text-red-500 border-red-500 border placeholder:text-red-300 ring-3 ring-red-300/90'
			: 'text-gray-700 focus-visible:border-primary-500 focus-visible:ring-3 ring-primary-300/90'
	]);

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
				this={label ? 'label' : 'p'}
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
	{@render children({ styleClasses, inputId, invalid })}
	{#if invalid && error && !inline}
		<p class="mt-1 text-xs text-red-500">{error}</p>
	{/if}
</div>
