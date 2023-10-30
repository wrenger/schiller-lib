<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { User } from "./UserView.svelte";
	import type { UserParams } from "./UserSearch.svelte";
	import List from "../../components/basic/List.svelte";

	export let promise: Promise<User[]>;
	export let params: UserParams;
	export let active: User | null;
	export let isNew: boolean;

	let list: List;

	export async function reload() {
		if (list) list.reloadList();
	}
</script>

<List
	bind:this={list}
	bind:active
	bind:isNew
	{promise}
	req={`api/user?query=${params?.input}${
		params?.permission != null ? `&may_borrow=${params?.permission}` : ""
	}`}
>
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
			<p class="mb-0 text-truncate">{item.forename} {item.surname}</p>
			<small class="text-muted text-truncate">{item.account}</small>
		</div>
		<div class="d-flex flex-column align-items-end">
			<p class="mb-0 text-truncate">{item.role}</p>
		</div>
	</button>
</List>
