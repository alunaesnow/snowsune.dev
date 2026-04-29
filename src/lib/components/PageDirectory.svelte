<script lang="ts">
	import { page } from '$app/state';

	let { title = 'Page Directory', pages }: { title?: string; pages: string[] } = $props();
</script>

<p class="text-center">{title}</p>
<div class="flex flex-col text-center">
	{#each pages as name (name)}
		<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
		<a href={(page.route.id + '/' + name).replace('//', '/')}>{name}</a>
	{/each}
</div>
