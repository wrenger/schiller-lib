<script lang="ts">
	import { _ } from "svelte-i18n";
	import type api from "$lib/api";

	type T = $$Generic<{}>;

	const CHUNK_SIZE: number = 250;

	export let active: T | null;
	export let isNew: boolean;
	export let load: (offset: number, limit: number) => Promise<api.Limited<T>>;
	export let key: (t: T) => string;

	let items: T[] = [];
	let total_count: number = 0;
	let ul: HTMLUListElement;

	let loading = false;
	let needsReload = false;

	async function loadMore() {
		if (!loading && items.length < total_count) {
			loading = true;

			const offset = items.length;
			let result = await load(offset, CHUNK_SIZE);
			total_count = result.total_count;
			items = [...items, ...result.rows];

			if (active != null) {
				let a = active;
				active = items.find((item) => key(a) == key(item)) || null;
			}

			if (needsReload) doReload();
			loading = false;
		}
	}

	async function doReload() {
		let scrollPosition = ul.scrollTop;

		let result = await load(0, Math.max(Math.ceil(items.length / CHUNK_SIZE) * CHUNK_SIZE, CHUNK_SIZE));
		total_count = result.total_count;
		items = result.rows;

		if (active != null) {
			let a = active;
			active = items.find((item) => key(a) == key(item)) || null;
		}

		requestAnimationFrame(() => {
			if (ul) ul.scrollTo(0, scrollPosition);
		});
	}

	export async function reload() {
		if (loading) {
			needsReload = true;
		} else {
			loading = true;
			doReload();
			loading = false;
		}
	}

	function handleScroll(event: { target: any }) {
		const target = event.target;
		const distanceToBottom = target.scrollHeight - target.scrollTop - target.clientHeight;

		if (distanceToBottom <= target.scrollHeight * 0.15) {
			loadMore();
		}
	}
</script>

<div class="card list">
	<slot name="header" />
	<ul bind:this={ul} class="list-group list-group-flush list-body" on:scroll={handleScroll}>
		{#each items as item (key(item))}
			<slot name="item" {item} class="list-group-item list-group-item-action" />
		{:else}
			<li class="list-group-item disabled">{$_(".error.none")}</li>
		{/each}
	</ul>
	<div class="card-footer d-flex justify-content-between align-items-center">
		{$_(".search.results", { values: { 0: items.length, 1: total_count } })}
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
