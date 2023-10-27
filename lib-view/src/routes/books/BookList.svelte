<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { Book } from "./BookView.svelte";
	import { request } from "$lib/util";
	import type { BookParams } from "./BookSearch.svelte";

	export let promise: Promise<Book[]>;
	export let params: BookParams;
	export let active: Book | null;
	export let isNew: boolean;

	const scrollThreshold = 2000;

	let items: Book[] | undefined = undefined;
	let loadingMore = false;
	let listLoaded = false;
	let startLoading = false;
	let scrollPosition = 0;
	let ul: HTMLUListElement;

	async function loadMore() {
		if (!loadingMore && !listLoaded && !startLoading) {
			loadingMore = true;
			const offset = items?.length || 0;
			try {
				const newItems = await request(
					`api/book?query=${params?.input}&offset=${offset}&limit=250`,
					"GET",
					null
				);
				items = items?.concat(newItems);
				promise = (items || []) as unknown as Promise<Book[]>;
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

		promise = request(`api/book?query=${params?.input}&limit=${items?.length}`, "GET", null);

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

	$: if (params || !params) {
		items = undefined;
		listLoaded = false;
	}
	$: if (promise instanceof Promise) {
		startLoading = true;
		promise.then((val) => {
			items = val;
			startLoading = false;
		});
	}
	$: if ((active || !active) && items)
		active = items.find((item) => active && item.id == active.id) || null;
</script>

<div class="card list">
	<div class="card-header d-flex justify-content-between">
		{$_(".book.title")} / {$_(".book.authors")}
		<span>{$_(".book.id")} / {$_(".book.state")}</span>
	</div>
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
				{#each data as item (item.id)}
					<button
						class="list-group-item list-group-item-action d-flex justify-content-between"
						class:active={item === active}
						id={item.id}
						on:click={() => {
							active = item;
						}}
					>
						<div class="d-flex flex-column">
							<p class="mb-0 text-truncate">{item.title}</p>
							<small class="text-muted text-truncate">{item.authors.join(", ")}</small>
						</div>
						<div class="d-flex flex-column align-items-end">
							<small class="text-muted text-truncate">{item.id}</small>
							<p class="mb-0 text-truncate">
								{!item.borrowable
									? `${$_(".book.not-borrowable")}`
									: item.borrower
									? `${$_(".book.borrowed")}`
									: item.reservation
									? `${$_(".book.reserved")}`
									: `${$_(".book.available")}`}
							</p>
						</div>
					</button>
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
			: `${$_(".book.request")}... `}
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
