<script lang="ts">
	import { page } from '$app/stores';
	import { _ } from 'svelte-i18n';
	import type api from '$lib/api';
	import { popup } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let input: string = '';
	onMount(() => (input = $page.url.searchParams.get('search') ?? ''));

	export let params: api.UserSearch = { query: input };

	let timer: NodeJS.Timeout | null = null;

	function handleInputDelayed() {
		if (timer) clearTimeout(timer);
		timer = setTimeout(() => (params.query = input), 500);
	}
</script>

<div class="input-group grid-cols-[1fr_auto] mb-2">
	<input
		type="search"
		placeholder={$_('.search.user.entry')}
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
	<label class="label">
		<span>{$_('.user.permission')}</span>
		<select class="select" aria-label={$_('.search.advanced')} bind:value={params.may_borrow}>
			<option value={null} selected>{$_('.action.select')}</option>
			<option value={true}>{$_('.user.may-borrow')}</option>
			<option value={false}>{$_('.user.may-not-borrow')}</option>
		</select>
	</label>
</div>
