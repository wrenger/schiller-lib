<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { category, state } from '$lib/store';
	import api from '../../lib/api';
	import Spinner from '../basic/Spinner.svelte';

	let items = $category;
	let selected: api.Category | null = null;

	let id: string;
	let name: string;
	let section: string;

	$: items = $category;

	function selectCategory(item: api.Category | null) {
		if (item) {
			id = item.id;
			name = item.name;
			section = item.section;
		} else {
			id = '';
			name = '';
			section = '';
		}
	}

	$: selectCategory(selected);

	let addResponse: Promise<void>;
	async function add() {
		await api.category_add({ id, name, section });
		await onChange();
	}

	let editResponse: Promise<void>;
	async function edit() {
		if (selected != null) {
			await api.category_update(selected.id, { id, name, section });
			await onChange();
		}
	}

	let deleteResponse: Promise<void>;
	async function del() {
		if (selected != null) {
			await api.category_delete(selected.id);
			await onChange();
		}
	}

	async function onChange() {
		selected = {
			id,
			name,
			section
		};
		await reload();
	}

	async function reload() {
		let data = await api.categories();
		category.set(data);
		state.set({});
		if (selected != null) {
			let sid = selected.id;
			selected = data.find((t) => t.id == sid) || null;
		}
	}
</script>

<label class="label">
	<span>{$_('.category')}</span>
	<select class="select" bind:value={selected}>
		<option selected={selected == null} value={null}>{$_('.action.add')}</option>
		{#each items as category (category.id)}
			<option selected={category.id === selected?.id} value={category}>
				{category.id} - {category.name} - {category.section}
			</option>
		{/each}
	</select>
</label>

<label class="label">
	<span>{$_('.category.id')}</span>
	<input class="input" type="text" placeholder={$_('.category.id')} bind:value={id} />
</label>

<label class="label">
	<span>{$_('.category.name')}</span>
	<input class="input" type="text" placeholder={$_('.category.name')} bind:value={name} />
</label>

<label class="label">
	<span>{$_('.category.section')}</span>
	<input class="input" type="text" placeholder={$_('.category.section')} bind:value={section} />
</label>

<div class="flex flex-wrap space-x-2">
	{#if selected == null}
		<button
			id="book-add-button"
			class="btn variant-filled-primary mt-2"
			type="button"
			on:click={() => (addResponse = add())}
		>
			<Spinner response={addResponse} />
			{$_('.action.add')}
		</button>
	{:else}
		<button
			id="book-confirm-button"
			type="button"
			class="btn variant-filled-primary mt-2"
			on:click={() => (editResponse = edit())}
		>
			<Spinner response={editResponse} />
			{$_('.action.apply')}
		</button>
		<button
			class="btn variant-filled-error mt-2"
			type="button"
			aria-expanded="false"
			on:click={() => (deleteResponse = del())}
		>
			<Spinner response={deleteResponse} />
			{$_('.action.delete')}
		</button>
	{/if}
</div>
