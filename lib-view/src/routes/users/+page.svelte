<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Layout from '../Layout.svelte';
	import api from '$lib/api';
	import VirtualList from '../../lib/components/ui/virtual-list/VirtualList.svelte';
	import UserSearch from './UserSearch.svelte';
	import { count } from '$lib/store';
	import UserSelect from './UserSelect.svelte';
	import UserItem, { HEIGHT } from './UserItem.svelte';
	import UserActions from './UserActions.svelte';
	import UserDisplay from './UserDisplay.svelte';

	let active: api.User | null;
	let search: api.UserSearch = { query: '', may_borrow: null, offset: 0, limit: 200 };
	let layout: Layout;
	// layout mobile display, won't work without binding open
	let open: boolean;

	let list: VirtualList<api.User> | null = null;

	$: if (search) list?.reload();

	function onChange(user: api.User | null) {
		// layout mobile display selection/deselection
		if (user == null) {
			layout?.setOpen(false);
		} else {
			layout?.setOpen(true);
		}
		active = user;
		list?.reload();
	}
</script>

<svelte:head>
	<title>{$_('.search.user')}</title>
	<meta name="description" content={$_('.search.user')} />
</svelte:head>

<Layout bind:this={layout} bind:open>
	<svelte:fragment slot="list-nav">
		<div class="flex h-full items-center justify-between px-4">
			<h1 class="text-xl font-bold">{$_('.search.user')}</h1>
			<UserSelect {onChange} bind:params={search} />
		</div>
	</svelte:fragment>
	<svelte:fragment slot="list">
		<div class="grid grid-rows-[72px_auto] overflow-scroll">
			<UserSearch bind:params={search} />
			<VirtualList
				bind:this={list}
				bind:active
				scrollClass="pb-2"
				rowHeight={HEIGHT}
				load={(offset, limit) => api.user_search({ ...search, offset, limit })}
				key={(user) => user.account}
				onLoad={(total) => {
					$count.users = total;
				}}
			>
				<UserItem
					slot="item"
					let:item
					user={item}
					active={active?.account === item.account}
					onClick={() => {
						active = item;
						layout?.setOpen(true);
					}}
				/>
			</VirtualList>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="display-nav">
		<UserActions user={active} {onChange} />
	</svelte:fragment>
	<svelte:fragment slot="display">
		{#if active}
			<UserDisplay user={active} />
		{/if}
	</svelte:fragment>
</Layout>
