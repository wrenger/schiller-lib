<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import UserList from "./UserList.svelte";
	import UserSearch from "./UserSearch.svelte";
	import UserView from "./UserView.svelte";
	import api from "$lib/api";

	let params: api.BookSearch;
	let active: api.User | null;
	let isNew: boolean;
	let promise: Promise<any>;
	let list: UserList;

	$: if (params != undefined)
		promise = api.user_search({
			...params,
			limit: 250
		});
</script>

<svelte:head>
	<title>{$_(".user")}</title>
	<meta name="description" content={$_(".user")} />
</svelte:head>

<Container>
	<span slot="list">
		<UserSearch bind:params />
		<UserList bind:this={list} bind:active bind:isNew {promise} {params} />
	</span>
	<UserView slot="view" bind:user={active} bind:isNew reload={list ? list.reload : undefined} />
</Container>
