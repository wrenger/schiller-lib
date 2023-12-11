<script lang="ts" context="module">
	export const HEIGHT: number = 64;
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

	$: classesActive = (cond: boolean) => (cond ? '!bg-primary-active-token' : '');
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="list-option {classesActive(active)}"
	id={book.id}
	on:click={onClick}
	style="border-radius: 12px;"
>
	<span class="flex-auto truncate w-[200px]"
		><dt>{book.title}</dt>
		<dd class="text-sm opacity-50">{book.authors}</dd></span
	>
	<span class="text-end truncate">
		<dd class="text-sm opacity-50">{book.id}</dd>
		<dt>{tr_borrow_state(book)}</dt>
	</span>
</div>
