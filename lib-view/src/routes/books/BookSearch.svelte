<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { page } from '$app/stores';
	import type api from '$lib/api';
	import { onMount } from 'svelte';
	import { Search } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';

	export let params: api.BookSearch;
	let input: string = '';

	onMount(() => {
		let query = $page.url.searchParams.get('search');
		if (query) {
			input = query;
			params.query = input;
		}
	});

	let timer: any | undefined = undefined;

	function handleInputDelayed() {
		clearTimeout(timer);
		timer = setTimeout(() => (params.query = input), 500);
	}
</script>

<div class="p-4">
	<div class="relative">
		<Search class="absolute left-2 top-3 h-4 w-4 text-muted-foreground" />
		<Input
			class="pl-8"
			type="search"
			placeholder={$_('.search.book.entry')}
			bind:value={input}
			on:input={handleInputDelayed}
		/>
	</div>
</div>
