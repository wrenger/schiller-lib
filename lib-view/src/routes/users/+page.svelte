<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import Container from '../../components/basic/Container.svelte';
	import UserSearch from './UserSearch.svelte';
	import UserView from './UserView.svelte';
	import UserItem, { HEIGHT } from './UserItem.svelte';
	import VirtualList from '../../components/basic/VirtualList.svelte';

	let params: api.BookSearch;
	let active: api.User | null;
	let adding: boolean = false;

	let list: VirtualList<api.User> | null = null;
	let view: UserView | null = null;

	$: if (params) list?.reload();
	$: if (!adding) list?.stopAdding();
	$: if (active != null) {
		view?.display(active);
		adding = false;
	}

	function onAdd() {
		adding = true;
		view?.create();
	}

	function onChange(user: api.User | null) {
		// don't deselect when closing adding
		if (!(adding && user == null)) {
			active = user;
		}
		adding = false;
		list?.reload();
	}
</script>

<svelte:head>
	<title>{$_('.user')}</title>
	<meta name="description" content={$_('.user')} />
</svelte:head>

<Container isActive={active != null || adding}>
	<span slot="list">
		<VirtualList
			bind:this={list}
			bind:active
			rowHeight={HEIGHT}
			{onAdd}
			load={(offset, limit) => api.user_search({ ...params, offset, limit })}
			key={(user) => user.account}
		>
			<div slot="header" class="p-2 pb-0">
				<UserSearch bind:params />
				<span class="flex">
					<span class="flex-auto font-bold">{$_('.user.name')} / {$_('.user.account')}</span>
					<span class="font-bold">{$_('.user.role')} </span>
				</span>
			</div>
			<UserItem
				slot="item"
				let:item
				user={item}
				active={active?.account === item.account}
				onClick={() => (active = item)}
			/>
		</VirtualList>
	</span>
	<div slot="view" class="h-full" hidden={!(active != null || adding)}>
		<UserView bind:this={view} {onChange} />
	</div>
</Container>
