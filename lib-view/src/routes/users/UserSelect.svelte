<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import {
		Autocomplete,
		popup,
		type AutocompleteOption,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	export let label = '';
	export let placeholder = '';
	export let readonly: boolean = false;
	export let value = '';

	let items: api.Limited<api.User>;
	let mounted: boolean = false;
	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start'
	};

	onMount(() => (mounted = true));

	function toACO(users: api.User[] | undefined): AutocompleteOption<string, {}>[] | undefined {
		if (users)
			return users.map((user) => ({
				label: `${user.forename} ${user.surname}`,
				value: user.account,
				keywords: `${user.forename} ${user.surname} ${user.account}`,
				meta: {}
			}));
	}

	async function onSelect(event: CustomEvent<AutocompleteOption<string>>) {
		value = event.detail.value;
	}

	async function fetch(value: string) {
		items = await api.user_search({ query: value, limit: 10 });
	}

	$: if (mounted) fetch(value);
</script>

<label class="label">
	{#if label}
		<span>{label}</span>
	{/if}
	<input
		class="input autocomplete"
		type="search"
		name="autocomplete-search"
		{readonly}
		bind:value
		{placeholder}
		use:popup={popupSettings}
	/>
	<div
		data-popup={readonly ? '' : 'popupAutocomplete'}
		class="card w-fit max-w-sm max-h-48 p-4 overflow-y-auto z-[500]"
	>
		<Autocomplete
			bind:input={value}
			options={toACO(items?.rows)}
			emptyState={$_('.error.none')}
			regionEmpty="p-2 pl-4 pr-4 opacity-50"
			on:selection={onSelect}
		/>
	</div>
</label>
