<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";

	export let promise: Promise<api.Limited<any>>;
	export let url: string;
	export let query: Record<string, any> = {};
	export let active: any | null;
	export let isNew: boolean;

	let items: api.Limited<any> | undefined = undefined;
	let loadingMore = false;
	let listLoaded = false;
	let startLoading = false;
	let scrollPosition = 0;
	let ul: HTMLUListElement;

	async function loadMore() {
		if (!loadingMore && !listLoaded && !startLoading) {
			loadingMore = true;
			const offset = items?.rows.length || 0;
			try {
				const newItems = await api.loadMore(url, { ...query, offset, limit: 250 });
				if (newItems.rows.length === 0) {
					listLoaded = true;
				} else {
					if (items) {
						items.rows = items?.rows.concat(newItems.rows) || [];
						items.total_count = newItems.total_count;
					}
					promise = items as unknown as Promise<api.Limited<any>>;
				}
			} catch (error) {
				throw error;
			} finally {
				loadingMore = false;
			}
		}
	}

	export async function reloadList() {
		scrollPosition = ul.scrollTop;

		promise = api.loadMore(url, {
			...query,
			limit:
				(Math.ceil((items?.rows.length ? items?.rows.length : 0) / 250) == 0
					? 1
					: Math.ceil((items?.rows.length ? items?.rows.length : 0) / 250)) * 250
		});

		items = undefined;
		listLoaded = false;

		promise.then(() => {
			requestAnimationFrame(() => {
				if (ul) ul.scrollTo(0, scrollPosition);
			});
		});
	}

	function handleScroll(event: { target: any }) {
		const target = event.target;
		const distanceToBottom = target.scrollHeight - target.scrollTop - target.clientHeight;

		if (distanceToBottom <= target.scrollHeight * 0.15) {
			loadMore();
		}
	}

	$: if (query || !query) {
		items = undefined;
		listLoaded = false;
	}

	$: if (promise instanceof Promise) {
		startLoading = true;
		promise.then((val) => {
			then(val);
		});
	}

	function then(val: any) {
		items = val;
		startLoading = false;
		active =
			val.rows.find(
				(item: { id: string; account: any }) =>
					(item.id && item.id == active?.id.trim()) ||
					(item.account && item.account == active?.account.trim())
			) || null;
	}
</script>

<div class="card list">
	<slot name="header" />
	<ul bind:this={ul} class="list-group list-group-flush list-body" on:scroll={handleScroll}>
		{#await promise}
			<li class="list-group-item">
				<div class="d-flex justify-content-center">
					<div class="spinner-grow" role="status">
						<span class="visually-hidden">Loading...</span>
					</div>
				</div>
			</li>
		{:then data}
			{#if data && data.rows}
				{#each data.rows as item (item.id ? item.id : item.account)}
					<slot name="item" {item} class="list-group-item list-group-item-action" />
				{:else}
					<li class="list-group-item disabled">{$_(".error.none")}</li>
				{/each}
			{/if}
			{#if loadingMore}
				<li class="list-group-item">
					<div class="d-flex justify-content-center">
						<div class="spinner-grow" role="status">
							<span class="visually-hidden">Loading...</span>
						</div>
					</div>
				</li>
			{/if}
		{/await}
	</ul>
	<div class="card-footer d-flex justify-content-between align-items-center">
		{Array.isArray(items?.rows)
			? $_(".search.results", { values: { 0: items?.rows.length, 1: items?.total_count } })
			: `${$_(".action.load")}... `}
		<button
			class="btn btn-outline-primary {isNew ? 'active' : ''}"
			type="button"
			title={$_(".book.new")}
			on:click={() => (isNew = true)}><i class="bi bi-plus-lg" /></button
		>
	</div>
</div>

<style>
	.list-group-item-action {
		cursor: pointer;
	}
	.list {
		--border-height: 45px;
		height: calc(100% - var(--border-height));
	}
	.list-body {
		overflow-y: scroll;
		flex: 1;
	}
</style>
