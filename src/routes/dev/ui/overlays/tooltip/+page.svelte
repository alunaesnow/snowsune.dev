<script lang="ts">
	import type { Placement } from '@floating-ui/dom';

	import { Tooltip } from '$lib/foxyui/components';

	const tooltips: { placement: Placement; icon: string }[] = [
		{ placement: 'top-end', icon: 'icon-[mingcute--arrow-left-up-line]' },
		{ placement: 'top', icon: 'icon-[mingcute--arrow-up-line]' },
		{ placement: 'top-start', icon: 'icon-[mingcute--arrow-right-up-line]' },
		{ placement: 'left', icon: 'icon-[mingcute--arrow-left-line]' },
		// spacer
		{ placement: 'right-end', icon: 'icon-[mingcute--question-line]' },
		{ placement: 'right', icon: 'icon-[mingcute--arrow-right-line]' },
		{ placement: 'bottom-end', icon: 'icon-[mingcute--arrow-left-down-line]' },
		{ placement: 'bottom', icon: 'icon-[mingcute--arrow-down-line]' },
		{ placement: 'bottom-start', icon: 'icon-[mingcute--arrow-right-down-line]' }
	];
</script>

<div class="mt-10">
	<div class="mx-auto grid w-fit grid-cols-3 grid-rows-3 gap-8">
		{#each tooltips as tooltip, index (index)}
			<Tooltip
				placement={tooltip.placement}
				text={tooltip.placement}
				class="flex h-8 w-8 items-center rounded-sm bg-gray-100 px-2"
			>
				<span class={tooltip.icon}></span>
			</Tooltip>
		{/each}
	</div>
</div>

<div class="mt-10">
	<Tooltip
		text="Id commodo consequat sint minim mollit sit. Adipisicing proident reprehenderit proident sit. Dolor deserunt irure duis ad aute exercitation sint elit aute. Commodo quis sunt occaecat ea aliqua commodo nostrud laborum. Incididunt est adipisicing enim aute qui reprehenderit adipisicing ut."
		class="bg-gray-100 px-2"
	>
		<p>Very very long tooltip</p>
	</Tooltip>
</div>
