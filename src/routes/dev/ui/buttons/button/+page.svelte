<script lang="ts">
	import { Button } from '$lib/foxyui/components';
</script>

<p>Buttons are buttons!</p>
<p>They have 2 variants, 'default' and 'filled':</p>

<Button variant="default">Default</Button>
<Button variant="filled">Filled</Button>

<p>You can use the `element` prop to change the element:</p>
<Button
	variant="filled"
	element="a"
	href="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
	target="_blank">I'm a link</Button
>

<p>You can add icons with `icon` and `iconSide`:</p>
<Button variant="filled" icon="i-[mingcute--airplane-line]" iconSide="left">iconSide left</Button>
<Button variant="filled" icon="i-[mingcute--aerial-lift-line]" iconSide="right"
	>iconSide right</Button
>

<p>rco adds some root classes (TODO):</p>
<Button rco="w-full">Full width button!</Button>
