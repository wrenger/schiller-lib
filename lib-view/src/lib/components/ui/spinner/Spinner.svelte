<script lang="ts">
	import { LoaderCircle } from 'lucide-svelte';

	export let response: Promise<any>;
	export let spinnerClass: string = '';
</script>

{#await response}
	<LoaderCircle class="{spinnerClass} mr-2 h-4 w-4 animate-spin" />
{:then}
	<slot />
{:catch}
	<slot />
{/await}
