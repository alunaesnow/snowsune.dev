<script lang="ts">
	import { Floater } from '..';
	import InputFrame from './InputFrame.svelte';
	import type { PassableInputFrameProps } from './InputFrame.svelte';

	type Props = {
		value?: Date;
		onchange?: (value: Date) => void;
		placeholder?: string;
	} & PassableInputFrameProps<Date | undefined>;

	let { value = $bindable(undefined), onchange, placeholder, ...inputFrameProps }: Props = $props();

	let pickerMode: 'day' | 'month' = $state('day');
	let floaterOpen = $state(false);

	let viewed = $state.raw(value ?? new Date());

	let renderDetails = $derived.by(() => {
		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		let clone = new Date(viewed.getTime());
		clone.setDate(1);
		let prevCount = Math.max(clone.getDay() - 1, 0);
		clone.setMonth(clone.getMonth() + 1);
		clone.setDate(0);
		let dateTill = clone.getDate();
		clone.setDate(0);
		let prevDateTill = clone.getDate();

		return { prevCount, dateTill, prevDateTill };
	});

	$effect(() => {
		if (value) {
			onchange?.(value);
		}
	});

	function pickDay(day: number) {
		value = new Date(viewed.getFullYear(), viewed.getMonth(), day);
		viewed = value;
		floaterOpen = false;
	}

	function pickMonth(month: number) {
		viewed = new Date(viewed.getFullYear(), month, viewed.getDate());
		pickerMode = 'day';
	}

	function modeButton() {
		if (pickerMode == 'day') {
			pickerMode = 'month';
		}
	}

	function prevButton() {
		if (pickerMode == 'day') {
			pickMonth(viewed.getMonth() - 1);
		} else {
			viewed = new Date(viewed.getFullYear() - 1, viewed.getMonth(), viewed.getDate());
		}
	}

	function nextButton() {
		if (pickerMode == 'day') {
			pickMonth(viewed.getMonth() + 1);
		} else {
			viewed = new Date(viewed.getFullYear() + 1, viewed.getMonth(), viewed.getDate());
		}
	}

	let buttonclass = 'p-2 text-center hover:bg-blue-500 hover:text-white rounded-sm cursor-pointer';
</script>

<InputFrame {value} {...inputFrameProps}>
	{#snippet children({ styleClasses, invalid })}
		<Floater
			bind:open={floaterOpen}
			onchange={(open) => {
				if (!open) {
					pickerMode = 'day';
				}
			}}
			closeOnClickOutside
			toggleOnClick
			focusTrap
			middlewares={{ arrowSize: 0, offset: 8 }}
			placement="bottom"
		>
			{#snippet floater()}
				<div class="p-4">
					<div class="flex items-center text-gray-900">
						<button
							aria-label="previous month"
							onclick={prevButton}
							class="rounded-sm p-2 py-1 text-xl hover:bg-gray-50"
						>
							<span class="icon-[mingcute--left-line] translate-y-0.5"></span>
						</button>
						<button onclick={modeButton} class="grow rounded-sm p-2 text-center hover:bg-gray-50">
							{#if pickerMode == 'day'}
								{viewed.toLocaleDateString(undefined, { month: 'long', year: 'numeric' })}
							{:else}
								{viewed.getFullYear()}
							{/if}
						</button>
						<button
							aria-label="next month"
							onclick={nextButton}
							class="rounded-sm p-2 py-1 text-xl hover:bg-gray-50"
						>
							<span class="icon-[mingcute--right-line] translate-y-0.5"></span>
						</button>
					</div>
					{#if pickerMode == 'day'}
						<div class="grid grid-cols-7 grid-rows-6 text-sm">
							{#each ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su'] as day (day)}
								<span class="p-2 text-center text-gray-400">{day}</span>
							{/each}
							{#each { length: renderDetails.prevCount }, preDay}
								<button
									class={'text-gray-300 ' + buttonclass}
									onclick={() => pickDay(-renderDetails.prevCount + preDay + 1)}
								>
									{renderDetails.prevDateTill - renderDetails.prevCount + preDay + 1}
								</button>
							{/each}
							{#each { length: renderDetails.dateTill }, day}
								<button class={'text-gray-800 ' + buttonclass} onclick={() => pickDay(day + 1)}>
									{day + 1}
								</button>
							{/each}
							{#each { length: 35 - renderDetails.prevCount - renderDetails.dateTill }, postDay}
								<button
									class={'text-gray-300 ' + buttonclass}
									onclick={() => pickDay(renderDetails.dateTill + postDay + 1)}
								>
									{postDay + 1}
								</button>
							{/each}
						</div>
					{:else if pickerMode == 'month'}
						<div class="mt-2 grid grid-cols-3 grid-rows-4 text-sm">
							{#each ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sept', 'Oct', 'Nov', 'Dec'] as month, index (month)}
								<button
									onclick={() => pickMonth(index)}
									class={'rounded-sm p-2 px-7 text-gray-800 ' +
										(index == viewed.getMonth()
											? 'bg-blue-500 text-white hover:bg-blue-600'
											: 'hover:bg-gray-50')}
								>
									{month}
								</button>
							{/each}
						</div>
					{/if}
				</div>
			{/snippet}

			<button class={styleClasses}>
				<span class="mr-2 icon-[mingcute--calendar-2-line] text-base"></span>
				<span class={invalid ? ' text-red-400' : ''}>
					{#if value}
						{value.toLocaleDateString()}
					{:else}
						{placeholder ?? 'pick date'}
					{/if}
				</span>
			</button>
		</Floater>
	{/snippet}
</InputFrame>
