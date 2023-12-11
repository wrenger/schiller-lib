<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	export let response: Promise<any>;
</script>

{#await response}
	<ProgressRadial
		stroke={100}
		meter="dark:stroke-surface-900 stroke-surface-50"
		track="dark:stroke-surface-900/30 stroke-surface-50/30"
		strokeLinecap={'round'}
		width={'w-5'}
		value={undefined}
		class="me-2"
	/>
{/await}
