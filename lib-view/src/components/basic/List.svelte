<script lang="ts">
	import { _ } from "svelte-i18n";
	import Request from "./Request.svelte";

	export let promise: Promise<any[]>;
	export let req: string;
	export let active: any | null;
	export let isNew: boolean;

	const scrollThreshold = 2000;

	let items: any[] | undefined = undefined;
	let loadingMore = false;
	let listLoaded = false;
	let startLoading = false;
	let scrollPosition = 0;
	let ul: HTMLUListElement;
	let r: Request;

	async function loadMore() {
		if (!loadingMore && !listLoaded && !startLoading) {
			loadingMore = true;
			const offset = items?.length || 0;
			try {
				const newItems = await r.request(`${req}&offset=${offset}&limit=250`, "GET", null);
				items = items?.concat(newItems);
				promise = (items || []) as unknown as Promise<any[]>;
				if (newItems?.length === 0) listLoaded = true;
			} catch (error) {
				console.error("Error loading more items", error);
			} finally {
				loadingMore = false;
			}
		}
	}

	export async function reloadList() {
		scrollPosition = ul.scrollTop;

		promise = r.request(
			`${req}&limit=${
				(Math.ceil((items?.length ? items?.length : 0) / 250) == 0
					? 1
					: Math.ceil((items?.length ? items?.length : 0) / 250)) * 250
			}`,
			"GET",
			null
		);

		items = undefined;
		listLoaded = false;

		promise.then(() => {
			requestAnimationFrame(() => ul.scrollTo(0, scrollPosition));
		});
	}

	function handleScroll(event: { target: any }) {
		const target = event.target;
		const distanceToBottom = target.scrollHeight - target.scrollTop - target.clientHeight;

		if (distanceToBottom <= scrollThreshold) {
			loadMore();
		}
	}

	$: if (req || !req) {
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
			val.find(
				(item: { id: any; account: any }) =>
					(item.id && item.id == active?.id) || (item.account && item.account == active?.account)
			) || null;
	}
</script>

<Request bind:this={r} />

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
			{#if data}
				{#each data as item (item.id ? item.id : item.account)}
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
		{Array.isArray(items)
			? $_(".search.results", { values: { 0: items?.length } })
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
