<script lang="ts">
	import { _ } from "svelte-i18n";
	import List from "../../components/basic/List.svelte";
	import type api from "$lib/api";

	export let promise: Promise<api.Limited<api.User>>;
	export let params: api.UserSearch;
	export let active: api.User | null;
	export let isNew: boolean;

	let list: List;

	export async function reload() {
		if (list) list.reloadList();
	}
</script>

<List bind:this={list} bind:active bind:isNew {promise} url="api/user" query={params}>
	<div slot="header" class="card-header d-flex justify-content-between">
		{$_(".user.name")} / {$_(".user.account")}
		<span>{$_(".user.role")} </span>
	</div>
	<button
		slot="item"
		let:item
		class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
		class:active={item === active}
		id={item.account}
		on:click={() => {
			active = item;
		}}
	>
		<div class="d-flex flex-column">
			<p class="mb-0 text-truncate q">{item.forename} {item.surname}</p>
			<small class="text-muted text-truncate q">{item.account}</small>
		</div>
		<div class="d-flex flex-column align-items-end">
			<p class="mb-0 text-truncate q">{item.role}</p>
		</div>
	</button>
</List>
