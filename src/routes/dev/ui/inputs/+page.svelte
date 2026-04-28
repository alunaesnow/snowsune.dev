<script lang="ts">
	import DateRangePicker from '$lib/components/ui/DateRangePicker.svelte';
	import { ComboBox, DatePicker, Form, NumberInput } from '$lib/foxyui/components';
	import SegmentedRatio from '$lib/foxyui/components/inputs/SegmentedRatio.svelte';

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

<div class="mx-auto w-64 space-y-10">
	<Form oncomplete={console.log} onerror={console.error}>
		{#snippet children({ submit })}
			<DatePicker id="date" validator={dateValidator} liveValidate></DatePicker>
			<NumberInput id="number" min={-1} max={10} placeholder="0" required liveValidate />
			<ComboBox
				id="fruit"
				values={['apple', 'grapes', 'pear', 'kiwi', 'banana']}
				required
				liveValidate
			/>

			<SegmentedRatio
				value="left"
				entries={[
					{ value: 'left', icon: 'i-[mingcute--list-check-3-line]' },
					{ value: 'center' },
					{ value: 'right' }
				]}
			/>

			<DateRangePicker />

			<button
				class="w-full rounded-sm bg-blue-500 p-2 font-semibold text-white shadow-sm hover:bg-blue-600 hover:shadow-lg"
				onclick={submit}>Submit</button
			>
		{/snippet}
	</Form>
</div>
