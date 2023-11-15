<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";
	import Container from "../../components/basic/Container.svelte";
	import List from "../../components/basic/List.svelte";
	import BookView from "./BookView.svelte";
	import BookSearch from "./BookSearch.svelte";
	import BookItem from "./BookItem.svelte";

	let params: api.BookSearch;
	let active: api.Book | null;
	let isNew: boolean;
	let list: List<api.Book> | null = null;

	$: if (params) list?.reload();
</script>

<svelte:head>
	<title>{$_(".search.book")}</title>
	<meta name="description" content={$_(".book")} />
</svelte:head>

<Container>
	<span slot="list">
		<BookSearch bind:params />
		<List
			bind:this={list}
			bind:active
			bind:isNew
			load={(offset, limit) => api.book_search({ ...params, offset, limit })}
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
		</List>
	</span>
	<BookView slot="view" bind:book={active} bind:isNew reload={list?.reload} />
</Container>
