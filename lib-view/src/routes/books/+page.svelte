<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import BookList from "./BookList.svelte";
	import type { BookParams } from "./BookSearch.svelte";
	import BookView, { Book } from "./BookView.svelte";
	import { request } from "$lib/util";
	import BookSearch from "./BookSearch.svelte";
	let params: BookParams;
	let active: Book | null;
	let isNew: boolean;
	let promise: Promise<any>;
	let t: BookList;

	$: if (params != undefined) promise = request(`api/book?query=${params?.input}&limit=250`, "GET", null);
</script>

<svelte:head>
	<title>{$_(".book")}</title>
	<meta name="description" content={$_(".book")} />
</svelte:head>

<Container>
	<span slot="list">
		<BookSearch bind:params />
		<BookList bind:this={t} bind:active bind:isNew {promise} {params} />
	</span>
	<BookView slot="view" bind:book={active} bind:isNew reload={t ? t.reloadList : undefined} />
</Container>
