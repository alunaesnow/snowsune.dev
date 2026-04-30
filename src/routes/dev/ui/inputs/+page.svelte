<script lang="ts">
	import { ComboBox, DatePicker, Form, NumberInput, Slider } from '$lib/foxyui/components';
	import Button from '$lib/foxyui/components/buttons/Button.svelte';
	import SegmentedRadio from '$lib/foxyui/components/inputs/SegmentedRadio.svelte';

	function dateValidator(value: Date | undefined): string | undefined {
		let now = new Date();
		if (value === undefined) {
			return 'please enter a date';
		} else if (value > now) {
			return 'date must be in the past';
		}

		return undefined;
	}
</script>

<div class="mx-auto w-64 space-y-8">
	<Form oncomplete={console.log} onerror={console.error}>
		{#snippet children({ submit })}
			<DatePicker
				id="date"
				validator={dateValidator}
				liveValidate
				title="Past date"
				description="Pick a date in the past"
			/>

			<NumberInput
				id="number"
				min={-1}
				max={10}
				placeholder="0"
				required
				liveValidate
				title="Number"
				description="Pick a number between -1 and 10"
			/>

			<ComboBox
				id="fruit"
				values={['apple', 'grapes', 'pear', 'kiwi', 'banana']}
				required
				liveValidate
				title="Favourite fruit"
				description="Pick your favourite fruit"
			/>

			<SegmentedRadio
				value="left"
				entries={[
					{ value: 'left', icon: 'icon-[mingcute--list-check-3-line]' },
					{ value: 'center' },
					{ value: 'right' }
				]}
				title="Text alignment"
				description="Pick a text alignment"
			/>

			<Slider
				value={2}
				marks={[0, 25, 50, 75, 100]}
				title="Another number"
				description="Pick a number between 0 and 100."
			/>

			<Button variant="filled" onclick={submit} rco="w-full">Submit</Button>
		{/snippet}
	</Form>
</div>
