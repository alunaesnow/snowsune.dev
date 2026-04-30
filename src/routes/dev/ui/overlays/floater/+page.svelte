<script lang="ts">
	import type { Placement } from '@floating-ui/dom';

	import { Button, Floater } from '$lib/foxyui/components';

	const floaters: { placement: Placement; icon: string }[] = [
		{ placement: 'top', icon: 'icon-[mingcute--arrow-up-line]' },
		{ placement: 'left', icon: 'icon-[mingcute--arrow-left-line]' },
		{ placement: 'right', icon: 'icon-[mingcute--arrow-right-line]' },
		{ placement: 'bottom', icon: 'icon-[mingcute--arrow-down-line]' }
	];

	let openA = $state(false);
	// svelte-ignore state_referenced_locally
	let isOpenA = $state(openA);

	let disabledB = $state(false);
	let isOpenB = $state(false);
</script>

<p>Floaters let you make floatin stuff'</p>
<Floater toggleOnClick class="inline-block">
	<Button>Floater</Button>
	{#snippet floater()}
		<p>Oh, why hello there, you have opened the floater!</p>
	{/snippet}
</Floater>

<p>
	You can manually controll a floater with the `open` prop, you should also set
	`closeOnClickOutside` to `false` to prevent the floater from instantly closing
</p>
<p>Use the onchange prop to listen for changes</p>
<p>The `open` prop is also bindable</p>
<Button onclick={() => (openA = !openA)}>Toggle floater</Button>
<Floater
	closeOnClickOutside={false}
	open={openA}
	onchange={(open) => (isOpenA = open)}
	class="inline-block bg-gray-100 p-2 px-3"
>
	<div>The floater is currently {isOpenA ? 'open' : 'closed'}</div>
	{#snippet floater()}
		<p>Hello</p>
	{/snippet}
</Floater>

<p>Set `toggleOnClick` to controll the floater with clicks</p>
<p>Set `openOnHover` to show the floater on hover</p>
<Floater openOnHover class="inline-block">
	<Button>I will open on hover</Button>
	{#snippet floater()}
		<p>Hello!</p>
	{/snippet}
</Floater>

<p>
	Floaters trap focus by default, set `data-autofocus` to determine which element will be focused by
	default
</p>
<Floater toggleOnClick class="inline-block">
	<Button>Floater with focus trap</Button>
	{#snippet floater()}
		<p>Try tab and shift-tab!</p>
		<Button variant="filled">First button</Button>
		<Button variant="filled" data-autofocus>Second button</Button>
	{/snippet}
</Floater>

<p>Set parent width to true to match the parent width! (usefull for dropdowns)</p>
<Floater toggleOnClick parentWidth class="inline-block">
	<Button>Parent width floater</Button>
	{#snippet floater()}
		<p>Huh, quite tight in here isn't it!</p>
	{/snippet}
</Floater>

<div class="mt-10">
	<div class="mx-auto grid w-fit grid-cols-2 grid-rows-2 gap-8">
		{#each floaters as f}
			<Floater
				toggleOnClick
				placement={f.placement}
				tabindex="0"
				class="flex h-8 w-8 items-center rounded-sm bg-gray-100 px-2"
			>
				<span class={f.icon}></span>
				{#snippet floater()}
					<p>Hey there! I'm on the {f.placement} side :)</p>
				{/snippet}
			</Floater>
		{/each}
	</div>
</div>

<p>Edit the `middlewares` option to change floating-ui middleware</p>

<p>Nested floater:</p>
<Floater toggleOnClick class="inline-block">
	<Button>Click to open</Button>
	{#snippet floater()}
		<p>Nested in here!!</p>
		<Floater toggleOnClick class="inline-block">
			<Button>Click me :D</Button>
			{#snippet floater()}
				<p>Oh hello!</p>
			{/snippet}
		</Floater>
	{/snippet}
</Floater>

<p>Set `disabled` to disable the popover</p>
<Button variant="filled" onclick={() => (disabledB = !disabledB)}>Disable</Button>
<Floater
	toggleOnClick
	closeOnClickOutside={false}
	onchange={(open) => (isOpenB = open)}
	disabled={disabledB}
	class="inline-block"
>
	<Button
		>We are currently {isOpenB ? 'open' : 'closed'} and {disabledB ? '' : 'not '}disabled</Button
	>
	{#snippet floater()}
		<p>Hi there!</p>
	{/snippet}
</Floater>
