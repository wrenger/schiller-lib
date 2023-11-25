<script lang="ts" context="module">
	export const HEIGHT: number = 62;
</script>

<script lang="ts">
	import type api from "$lib/api";
	import { _ } from "svelte-i18n";

	export let book: api.Book;
	export let active: boolean = false;
	export let onClick: () => void;

	function tr_borrow_state(book: api.Book): string {
		if (!book.borrowable) return $_(".book.not-borrowable");
		if (book.borrower) return $_(".book.borrowed");
		if (book.reservation) return $_(".book.reserved");
		return $_(".book.available");
	}
</script>

<button
	class="list-group-item list-group-item-action d-flex justify-content-between"
	class:active
	id={book.id}
	on:click={onClick}
>
	<div class="d-flex flex-column">
		<p class="mb-0 text-truncate q">{book.title}</p>
		<small class="text-muted text-truncate q">{book.authors.join(", ")}</small>
	</div>
	<div class="d-flex flex-column align-items-end">
		<small class="text-muted text-truncate q">{book.id}</small>
		<p class="mb-0 text-truncate q">
			{tr_borrow_state(book)}
		</p>
	</div>
</button>
