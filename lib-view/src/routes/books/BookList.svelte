<script lang="ts">
	import { _ } from "svelte-i18n";
	import List from "../../components/basic/List.svelte";
	import type api from "$lib/api";

	export let promise: Promise<api.Limited<api.Book>>;
	export let params: api.BookSearch;
	export let active: api.Book | null;
	export let isNew: boolean;

	let list: List;

	export async function reload() {
		if (list) list.reloadList();
	}
</script>

<List bind:this={list} bind:active bind:isNew {promise} url="api/book" query={params}>
	<div slot="header" class="card-header d-flex justify-content-between">
		{$_(".book.title")} / {$_(".book.authors")}
		<span>{$_(".book.id")} / {$_(".book.state")}</span>
	</div>
	<button
		slot="item"
		let:item
		class="list-group-item list-group-item-action d-flex justify-content-between"
		class:active={item === active}
		id={item.id}
		on:click={() => {
			active = item;
		}}
	>
		<div class="d-flex flex-column">
			<p class="mb-0 text-truncate q">{item.title}</p>
			<small class="text-muted text-truncate q">{item.authors.join(", ")}</small>
		</div>
		<div class="d-flex flex-column align-items-end">
			<small class="text-muted text-truncate q">{item.id}</small>
			<p class="mb-0 text-truncate q">
				{!item.borrowable
					? `${$_(".book.not-borrowable")}`
					: item.borrower
					? `${$_(".book.borrowed")}`
					: item.reservation
					? `${$_(".book.reserved")}`
					: `${$_(".book.available")}`}
			</p>
		</div>
	</button>
</List>
