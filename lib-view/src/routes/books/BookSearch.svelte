<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { page } from '$app/stores';
	import type api from '$lib/api';
	import CategorySelect from '../../components/basic/CategorySelect.svelte';
	import { popup } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let input: string = '';
	onMount(() => (input = $page.url.searchParams.get('search') ?? ''));

	export let params: api.BookSearch = { query: input };

	let timer: NodeJS.Timeout | undefined = undefined;

	function handleInputDelayed() {
		clearTimeout(timer);
		timer = setTimeout(() => (params.query = input), 500);
	}
</script>

<div class="input-group grid-cols-[1fr_auto] mb-2">
	<input
		type="search"
		placeholder={$_('.search.book.entry')}
		bind:value={input}
		on:input={handleInputDelayed}
	/>
	<button
		class="variant-soft"
		title={$_('.search.advanced')}
		use:popup={{ event: 'click', target: 'settings', placement: 'bottom-end' }}
	>
		<i class="fa-solid fa-wrench"></i>
	</button>
</div>
<div class="card p-4 w-60 shadow-xl z-[500]" data-popup="settings">
	<CategorySelect bind:value={params.category} label={$_('.category')} />
	<hr class="!my-4" />
	<label class="label">
		<span>{$_('.book.state')}</span>
		<select class="select" bind:value={params.state}>
			<option value={'None'} selected>{$_('.action.select')}</option>
			<option value={'Borrowable'}>{$_('.book.borrowable')}</option>
			<option value={'NotBorrowable'}>{$_('.book.not-borrowable')}</option>
			<option value={'Borrowed'}>{$_('.book.borrowed')}</option>
			<option value={'Reserved'}>{$_('.book.reserved')}</option>
		</select>
	</label>
</div>
