<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";
	import Container from "../../components/basic/Container.svelte";
	import UserSearch from "./UserSearch.svelte";
	import UserView from "./UserView.svelte";
	import UserItem from "./UserItem.svelte";
	import List from "../../components/basic/List.svelte";

	let params: api.BookSearch;
	let active: api.User | null;
	let isNew: boolean;
	let list: List<api.User> | null = null;

	$: if (params) list?.reload();
</script>

<svelte:head>
	<title>{$_(".user")}</title>
	<meta name="description" content={$_(".user")} />
</svelte:head>

<Container>
	<span slot="list">
		<UserSearch bind:params />
		<List
			bind:this={list}
			bind:active
			bind:isNew
			load={(offset, limit) => api.user_search({ ...params, offset, limit })}
			key={(user) => user.account}
		>
			<div slot="header" class="card-header d-flex justify-content-between">
				{$_(".user.name")} / {$_(".user.account")}
				<span>{$_(".user.role")} </span>
			</div>
			<UserItem
				slot="item"
				let:item
				user={item}
				active={active?.account === item.account}
				onClick={() => (active = item)}
			/>
		</List>
	</span>
	<UserView slot="view" bind:user={active} bind:isNew reload={list ? list.reload : undefined} />
</Container>
