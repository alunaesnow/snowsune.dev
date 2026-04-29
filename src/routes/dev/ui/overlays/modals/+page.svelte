<script lang="ts">
	import { Button, getModalsContext } from '$lib/foxyui/components';
	import TestModal from './TestModal.svelte';

	let value: string | null = $state('');

	const modals = getModalsContext();

	async function openModal() {
		value = await modals.open(TestModal, {});
	}
</script>

<p>Foxyui comes with a modal system! The ModalManager component should wrap the root of the app.</p>
<p>
	Modals can be opened using the `modals.open()` function of the modals context
	(`getModalsContext()`). The first parameter is the component to open as a modal, the second is the
	props to pass to the component and the third is a set of options. These include enabling focus
	trapping, close on click outside, close on escape key press, and a couple others. The modal
	component can have `close` and `submit` props, which will be provided by the modal manager. The
	close prop is a function that will close the modal, and the submit closes the modal, in addition
	to submitting anything passed in. when a modal is opened a promise is returned that resolves when
	the modal is closed, with the submitted data, or `null` if no data is present. it will not reject.
	only one modal can be opened at a time, as nested modals are considered bad ui design.
</p>
<Button onclick={openModal}>Open modal</Button>
<p>The submission was: {value}</p>
