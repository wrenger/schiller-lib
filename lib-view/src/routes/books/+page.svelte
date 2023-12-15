<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import Container from '../../components/basic/Container.svelte';
	import BookView from './BookView.svelte';
	import BookSearch from './BookSearch.svelte';
	import BookItem, { HEIGHT } from './BookItem.svelte';
	import VirtualList from '../../components/basic/VirtualList.svelte';

	let active: api.Book | null;
	let search: api.BookSearch;
	let adding = false;

	let list: VirtualList<api.Book> | null = null;
	let view: BookView | null = null;

	$: if (search) list?.reload();
	$: if (!adding) list?.stopAdding();
	$: if (active != null) {
		view?.display(active);
		adding = false;
	}

	// using a callback here, because two bidirectional bindings (active and adding) lead to race conditions.
	function onAdd() {
		adding = true;
		view?.create();
	}

	function onChange(book: api.Book | null) {
		// don't deselect when closing adding
		if (!(adding && book == null)) {
			active = book;
		}
		adding = false;
		list?.reload();
	}
</script>

<svelte:head>
	<title>{$_('.search.book')}</title>
	<meta name="description" content={$_('.book')} />
</svelte:head>

<Container isActive={active != null || adding}>
	<span slot="list">
		<VirtualList
			bind:this={list}
			bind:active
			rowHeight={HEIGHT}
			{onAdd}
			load={(offset, limit) => api.book_search({ ...search, offset, limit })}
			key={(book) => book.id}
		>
			<div slot="header" class="pt-2 pb-0">
				<BookSearch bind:params={search} />
				<span class="flex pr-2 pl-2">
					<span class="flex-auto font-bold">{$_('.book.title')} / {$_('.book.authors')}</span>
					<span class="font-bold">{$_('.book.id')} / {$_('.book.state')}</span>
				</span>
			</div>
			<BookItem
				slot="item"
				let:item
				book={item}
				active={active?.id === item.id}
				onClick={() => (active = item)}
			/>
		</VirtualList>
	</span>
	<div slot="view" class="h-full" hidden={!(active != null || adding)}>
		<BookView bind:this={view} {onChange} />
	</div>
</Container>
