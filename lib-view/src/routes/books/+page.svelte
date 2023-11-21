<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";
	import Container from "../../components/basic/Container.svelte";
	import BookView from "./BookView.svelte";
	import BookSearch from "./BookSearch.svelte";
	import BookItem, { HEIGHT } from "./BookItem.svelte";
	import VirtualList from "../../components/basic/VirtualList.svelte";

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
	function add() {
		adding = true;
		view?.adding();
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
	<title>{$_(".search.book")}</title>
	<meta name="description" content={$_(".book")} />
</svelte:head>

<Container>
	<span slot="list">
		<BookSearch bind:params={search} />
		<VirtualList
			bind:this={list}
			bind:active
			rowHeight={HEIGHT}
			{add}
			load={(offset, limit) => api.book_search({ ...search, offset, limit })}
			key={(book) => book.id}
		>
			<div slot="header" class="card-header d-flex justify-content-between">
				{$_(".book.title")} / {$_(".book.authors")}
				<span>{$_(".book.id")} / {$_(".book.state")}</span>
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
	<div slot="view" hidden={!(active != null || adding)}>
		<BookView bind:this={view} {onChange} />
	</div>
</Container>
