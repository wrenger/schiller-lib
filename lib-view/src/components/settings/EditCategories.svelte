<script lang="ts">
	import { _ } from "svelte-i18n";
	import { category, state } from "$lib/store";
	import Dialog from "../basic/Dialog.svelte";
	import Spinner from "../basic/Spinner.svelte";
	import api from "$lib/api";

	let dialog: Dialog;
	let items = $category;
	let selected: api.Category | null = null;

	let id: string;
	let name: string;
	let section: string;

	$: items = $category;

	export function open() {
		dialog.open();
	}

	function selectCategory(item: api.Category | null) {
		if (item) {
			id = item.id;
			name = item.name;
			section = item.section;
		} else {
			id = "";
			name = "";
			section = "";
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
		if (selected != null) {
			let sid = selected.id;
			selected = data.find((t) => t.id == sid) || null;
		}
		state.set({});
	}
</script>

<Dialog bind:this={dialog}>
	<h5 slot="header" class="m-0">{$_(".category.edit")}</h5>

	<span slot="body">
		<div class="row">
			<div class="col">
				<select class="form-select" id="categorySelect" bind:value={selected}>
					<option selected={selected == null} value={null}>{$_(".action.add")}</option>
					{#each items as category (category.id)}
						<option selected={category.id === selected?.id} value={category}>
							{category.id} - {category.name} - {category.section}
						</option>
					{/each}
				</select>
			</div>
		</div>
		<div class="mt-2">
			<label for="id" class="form-label">{$_(".category.id")}</label>
			<input
				type="text"
				placeholder={$_(".category.id")}
				class="form-control"
				id="id"
				bind:value={id}
			/>
		</div>
		<div class="mt-2">
			<label for="name" class="form-label">{$_(".category.name")}</label>
			<input
				type="text"
				placeholder={$_(".category.name")}
				class="form-control"
				id="name"
				bind:value={name}
			/>
		</div>
		<div class="mt-2">
			<label for="section" class="form-label">{$_(".category.section")}</label>
			<input
				type="text"
				placeholder={$_(".category.section")}
				class="form-control"
				id="section"
				bind:value={section}
			/>
		</div>
		<button
			id="book-add-button"
			class="btn btn-outline-primary mt-2"
			type="button"
			hidden={selected != null}
			on:click={() => (addResponse = add())}
		>
			<Spinner response={addResponse} />
			{$_(".action.add")}
		</button>
		<button
			id="book-confirm-button"
			type="button"
			class="btn btn-outline-primary mt-2"
			hidden={selected == null}
			on:click={() => (editResponse = edit())}
		>
			<Spinner response={editResponse} />
			{$_(".action.apply")}
		</button>
		<button
			class="btn btn-outline-danger mt-2"
			type="button"
			aria-expanded="false"
			hidden={selected == null}
			on:click={() => (deleteResponse = del())}
		>
			<Spinner response={deleteResponse} />
			{$_(".action.delete")}
		</button>
	</span>
</Dialog>
