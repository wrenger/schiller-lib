<script lang="ts" context="module">
	export const HEIGHT: number = 74;
</script>

<script lang="ts">
	import type api from '$lib/api';
	import { _ } from 'svelte-i18n';

	export let book: api.Book;
	export let active: boolean = false;
	export let onClick: () => void;

	function tr_borrow_state(book: api.Book): string {
		if (!book.borrowable) return $_('.book.not-borrowable');
		if (book.borrower) return $_('.book.borrowed');
		if (book.reservation) return $_('.book.reserved');
		return $_('.book.available');
	}
</script>

<button
	class="hover:bg-accent flex h-[66px] w-full flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all"
	class:bg-muted={active}
	id={book.id}
	on:click={onClick}
>
	<div class="grid w-full grid-cols-[1fr_auto] gap-1">
		<div class="truncate font-semibold">{book.title}</div>
		<div class="text-muted-foreground ml-auto text-nowrap text-xs">{book.id}</div>
		<div class="truncate text-xs font-medium">{book.authors}</div>
		<div
			class="ml-auto text-nowrap text-xs"
			class:text-destructive={!book.borrowable || book.borrower || book.reservation}
		>{tr_borrow_state(book)}</div>
	</div>
</button>
