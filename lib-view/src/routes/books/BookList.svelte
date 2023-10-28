<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { Book } from "./BookView.svelte";
	import type { BookParams } from "./BookSearch.svelte";
	import List from "../../components/basic/List.svelte";

	export let promise: Promise<Book[]>;
	export let params: BookParams;
	export let active: Book | null;
	export let isNew: boolean;

	let list: List;

	export async function reload() {
		if (list) list.reloadList();
	}
	
</script>

<List bind:this={list} bind:active bind:isNew {promise} req={`api/book?query=${params?.input}`}>
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
			<p class="mb-0 text-truncate">{item.title}</p>
			<small class="text-muted text-truncate">{item.authors.join(", ")}</small>
		</div>
		<div class="d-flex flex-column align-items-end">
			<small class="text-muted text-truncate">{item.id}</small>
			<p class="mb-0 text-truncate">
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
