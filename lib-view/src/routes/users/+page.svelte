<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import UserList from "./UserList.svelte";
	import UserSearch, { UserParams } from "./UserSearch.svelte";
	import type { User } from "./UserView.svelte";
	import UserView from "./UserView.svelte";
	import Request from "../../components/basic/Request.svelte";

	let params: UserParams;
	let active: User | null;
	let isNew: boolean;
	let promise: Promise<any>;
	let list: UserList;
	let r: Request;

	$: if (params != undefined)
		promise = r.request(
			`api/user?query=${params?.input}${
				params?.permission != null ? `&may_borrow=${params?.permission}` : ""
			}&limit=250`,
			"GET",
			null
		);
</script>

<svelte:head>
	<title>{$_(".user")}</title>
	<meta name="description" content={$_(".user")} />
</svelte:head>

<Request bind:this={r} />

<Container>
	<span slot="list">
		<UserSearch bind:params />
		<UserList bind:this={list} bind:active bind:isNew {promise} {params} />
	</span>
	<UserView slot="view" bind:user={active} bind:isNew reload={list ? list.reload : undefined} />
</Container>
