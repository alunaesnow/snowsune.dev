<script lang="ts">
	import { ComboBox, Form } from '$lib/foxyui/components';
	import Button from '$lib/foxyui/components/buttons/Button.svelte';

	let { close, submit }: { close: () => void; submit: (value: string) => void } = $props();

	function oncomplete(data: { fruit: string }) {
		submit(data.fruit);
	}
</script>

<div class="rounded-xl bg-white p-8 shadow-lg">
	<Form {oncomplete}>
		{#snippet children({ submit: formSubmit })}
			<p>Hello im the test modal, please choose a fruit:</p>

			<ComboBox id="fruit" values={['apple', 'grapes', 'pear', 'kiwi', 'banana']} required />

			<Button variant="filled" onclick={formSubmit}>Submit</Button>
			<Button onclick={close}>Close</Button>
		{/snippet}
	</Form>
</div>
