<script lang="ts">
	import { _ } from "svelte-i18n";
	import type api from "$lib/api";
	import { afterUpdate } from "svelte";

	type T = $$Generic<{}>;

	const CHUNK_SIZE: number = 250;

	export let active: T | null;
	export let add: () => void;
	export let load: (offset: number, limit: number) => Promise<api.Limited<T>>;
	export let key: (t: T) => string;

	let items: T[] = [];
	let total_count: number = 0;
	let row_height: number = 0;
	let element: HTMLDivElement;
	let body: HTMLDivElement;

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
		let result = await load(
			0,
			Math.max(Math.ceil(items.length / CHUNK_SIZE) * CHUNK_SIZE, CHUNK_SIZE)
		);
		total_count = result.total_count;
		items = result.rows;

		if (active != null) {
			let a = active;
			active = items.find((item) => key(a) == key(item)) || null;
		}
	}

	afterUpdate(() => {
		if (row_height === 0 && items.length > 0) {
			let child = body.children.item(0);
			if (child !== null) {
				row_height = child.clientHeight;
				if (element) element.scrollTo(0, element.scrollTop);
			}
		}
	});

	export async function reload() {
		if (loading) {
			needsReload = true;
		} else {
			loading = true;
			doReload();
			loading = false;
		}
	}

	function handleScroll(event: UIEvent & { currentTarget: EventTarget & HTMLDivElement }) {
		const target = event.currentTarget;
		const scrollBottom = target.scrollTop + target.clientHeight;

		if (scrollBottom > items.length * row_height * 0.8) {
			loadMore();
		}
	}
</script>

<div class="card list">
	<slot name="header" />
	<div bind:this={element} class="list-group list-group-flush list-body" on:scroll={handleScroll}>
		<div bind:this={body} style="min-height: {row_height * total_count}px;">
			{#each items as item (key(item))}
				<slot
					name="item"
					{item}
					class="list-group-item list-group-item-action"
					style="height: {row_height}px;"
				/>
			{:else}
				<div class="list-group-item disabled">{$_(".error.none")}</div>
			{/each}
		</div>
	</div>
	<div class="card-footer d-flex justify-content-between align-items-center">
		{$_(".search.results", { values: { 0: items.length, 1: total_count } })}
		<button class="btn btn-outline-primary" type="button" title={$_(".book.new")} on:click={add}
			><i class="bi bi-plus-lg" /></button
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
