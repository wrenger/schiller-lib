<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import BookList from "./BookList.svelte";
	import type { BookParams } from "./BookSearch.svelte";
	import BookView, { Book } from "./BookView.svelte";
	import BookSearch from "./BookSearch.svelte";
	import Request from "../../components/basic/Request.svelte";
	let params: BookParams;
	let active: Book | null;
	let isNew: boolean;
	let promise: Promise<any>;
	let list: BookList;
	let r: Request;

	$: if (params != undefined)
		promise = r.request(
			`api/book?query=${params?.input}${params?.category != null ? `&category=${params?.category}` : ""}${
				params?.status ? `&state=${params?.status}` : ""
			}
			&limit=250`,
			"GET",
			null
		);
</script>

<svelte:head>
	<title>{$_(".book")}</title>
	<meta name="description" content={$_(".book")} />
</svelte:head>

<Request bind:this={r} />

<Container>
	<span slot="list">
		<BookSearch bind:params />
		<BookList bind:this={list} bind:active bind:isNew {promise} {params} />
	</span>
	<BookView slot="view" bind:book={active} bind:isNew reload={list ? list.reload : undefined} />
</Container>
