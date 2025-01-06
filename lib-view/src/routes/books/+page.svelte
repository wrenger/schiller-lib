<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Layout from '../Layout.svelte';
	import api from '$lib/api';
	import VirtualList from '../../lib/components/ui/virtual-list/VirtualList.svelte';
	import BookSearch from '../books/BookSearch.svelte';
	import BookSearchHeader from './BookSearchHeader.svelte';
	import BookItem, { HEIGHT } from './BookItem.svelte';
	import BookActions from './BookActions.svelte';
	import BookDisplay from './BookDisplay.svelte';
	import { bookState } from "$lib/store";

	let active = $bookState.active;
	let search = $bookState.search;
	let scroll = $bookState.scroll;
	$: bookState.set({ scroll, active, search });

	let layout: Layout;
	// layout mobile display, won't work without binding open
	let open: boolean;

	let list: VirtualList<api.Book> | null = null;

	$: if (search) list?.reload();

	function onChange(book: api.Book | null) {
		// layout mobile display selection/deselection
		if (book == null) {
			layout?.setOpen(false);
		} else {
			layout?.setOpen(true);
		}
		active = book;
		list?.reload();
	}
</script>

<svelte:head>
	<title>{$_('.search.book')}</title>
	<meta name="description" content={$_('.search.book')} />
</svelte:head>

<Layout bind:this={layout} bind:open>
	<svelte:fragment slot="list-nav">
		<div class="flex h-full items-center justify-between px-4">
			<h1 class="text-xl font-bold">{$_('.search.book')}</h1>
			<BookSearchHeader {onChange} bind:params={search} />
		</div>
	</svelte:fragment>
	<svelte:fragment slot="list">
		<div class="grid grid-rows-[72px_auto] overflow-scroll">
			<BookSearch bind:params={search} />
			<VirtualList
				bind:this={list}
				bind:active
				bind:scroll
				scrollClass="pb-2"
				rowHeight={HEIGHT}
				load={(offset, limit) => api.book_search({ ...search, offset, limit })}
				key={(book) => book.id}
			>
				<BookItem
					slot="item"
					let:item
					book={item}
					active={active?.id === item.id}
					onClick={() => {
						active = item;
						layout?.setOpen(true);
					}}
				/>
			</VirtualList>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="display-nav">
		<BookActions book={active} {onChange} />
	</svelte:fragment>
	<svelte:fragment slot="display">
		{#if active}
			<BookDisplay book={active} />
		{/if}
	</svelte:fragment>
</Layout>
