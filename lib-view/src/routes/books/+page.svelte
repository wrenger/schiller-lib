<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import BookList from "./BookList.svelte";
	import BookView from "./BookView.svelte";
	import BookSearch from "./BookSearch.svelte";
	import api from "$lib/api";

	let params: api.BookSearch;
	let active: api.Book | null;
	let isNew: boolean;
	let promise: Promise<api.Limited<api.Book>>;
	let list: BookList;

	$: if (params != undefined) {
		promise = api.book_search({
			...params,
			limit: 250
		});
	}
</script>

<svelte:head>
	<title>{$_(".search.book")}</title>
	<meta name="description" content={$_(".book")} />
</svelte:head>

<Container>
	<span slot="list">
		<BookSearch bind:params />
		<BookList bind:this={list} bind:active bind:isNew {promise} {params} />
	</span>
	<BookView slot="view" bind:book={active} bind:isNew reload={list ? list.reload : undefined} />
</Container>
