<script lang="ts">
	import { Button } from '$lib/foxyui/components';
</script>

<p>Buttons are buttons!</p>
<p>These are the available variants:</p>

<Button variant="default">Default</Button>
<Button variant="filled">Filled</Button>
<Button variant="light">Light</Button>
<Button variant="transparent">Transparent</Button>

<p>You can use the `element` prop to change the element:</p>
<Button
	variant="filled"
	element="a"
	href="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
	target="_blank">I'm a link</Button
>

<p>You can add icons with `icon` and `iconSide`:</p>
<Button variant="filled" icon="icon-[mingcute--airplane-line]" iconSide="left">iconSide left</Button
>
<Button variant="filled" icon="icon-[mingcute--aerial-lift-line]" iconSide="right"
	>iconSide right</Button
>

<p>rco adds some root classes (TODO):</p>
<Button rco="w-full">Full width button!</Button>

<p>You can disable buttons with the `disabled` prop:</p>
<Button variant="default" disabled>Default</Button>
<Button variant="filled" disabled>Filled</Button>
<Button variant="light" disabled>Light</Button>
<Button variant="transparent" disabled>Transparent</Button>

<p>Button with only an icon:</p>
<Button icon="icon-[ri--heart-line]"></Button>
<Button icon="icon-[ri--heart-line]" variant="filled"></Button>
<Button icon="icon-[ri--heart-line]" variant="light"></Button>
<Button icon="icon-[ri--heart-line]" variant="transparent"></Button>
