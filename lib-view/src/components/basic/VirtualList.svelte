<script lang="ts">
	import { _ } from "svelte-i18n";
	import type api from "$lib/api";

	type T = $$Generic<{}>;

	const CHUNK_SIZE: number = 200;

	export let rowHeight: number;
	export let active: T | null;
	export let onAdd: () => void;
	export let load: (offset: number, limit: number) => Promise<api.Limited<T>>;
	export let key: (t: T) => string;

	let chunks: (T[] | null)[] = [];
	let firstChunk = -1;
	let lastChunk = -1;
	let totalCount: number = 0;

	let needsReload = false;
	let adding = false;

	let scroller: HTMLDivElement | null;

	export function stopAdding() {
		adding = false;
	}

	export async function reload() {
		needsReload = true;
		updateChunks();
	}

	async function updateChunks() {
		if (scroller == null) return;

		// calculate viewport
		const border = (rowHeight * CHUNK_SIZE) / 2;
		const top = Math.max(scroller.scrollTop - border, 0);
		const bottom = scroller.scrollTop + scroller.clientHeight + border;

		let first = rowHeight === 0 ? 0 : Math.floor(top / (CHUNK_SIZE * rowHeight));
		let last = rowHeight === 0 ? 0 : Math.floor(bottom / (CHUNK_SIZE * rowHeight));

		if (!needsReload && first === firstChunk && last === lastChunk) return;

		firstChunk = first;
		lastChunk = last;

		// add at least one
		if (chunks.length === 0) {
			chunks = [null];
		}

		// load chunks, grow if too short
		for (let i = 0; i < chunks.length; i++) {
			if (firstChunk <= i && i <= lastChunk) {
				if (chunks[i] == null || needsReload) {
					let { rows, total_count } = await load(i * CHUNK_SIZE, CHUNK_SIZE);
					if (totalCount !== total_count) {
						totalCount = total_count;
						for (let j = chunks.length; j < totalCount / CHUNK_SIZE; j++) {
							chunks.push(null);
						}
					}
					chunks[i] = rows;
				}
			} else if (chunks[i] != null) {
				chunks[i] = null;
			}
		}

		// truncate if too long
		let maxLen = Math.ceil(totalCount / CHUNK_SIZE);
		if (chunks.length > maxLen) {
			chunks = chunks.slice(0, maxLen);
		}

		// update active element
		if (needsReload) {
			if (active != null) {
				let a = key(active);
				for (let i = firstChunk; i <= lastChunk; i++) {
					let row = chunks[i]?.find((v) => key(v) == a);
					if (row) {
						active = row;
						break;
					}
				}
			}
			needsReload = false;
		}
	}
</script>

<div class="card list">
	<slot name="header" />
	<div bind:this={scroller} class="list-body" on:scroll={updateChunks}>
		<div
			class="list-group list-group-flush"
			style="min-height: {rowHeight * totalCount}px; max-height: {rowHeight *
				totalCount}px; position: relative;"
		>
			{#each chunks as chunk, i (i)}
				{#if chunk}
					<div
						style="position: absolute; top: {i * rowHeight * CHUNK_SIZE}px; left: 0; right: 0;"
						class="list-group list-group-flush"
					>
						{#each chunk as item (key(item))}
							<slot name="item" {item} class="list-group-item list-group-item-action" />
						{/each}
					</div>
				{/if}
			{:else}
				<div class="list-group-item disabled">{$_(".error.none")}</div>
			{/each}
		</div>
	</div>
	<div class="card-footer d-flex justify-content-between align-items-center">
		{$_(".search.results", { values: { 0: totalCount } })}
		<button
			class="btn btn-outline-primary"
			class:active={adding}
			type="button"
			title={$_(".book.new")}
			on:click={() => {
				adding = true;
				onAdd();
			}}
		>
			<i class="bi bi-plus-lg" />
		</button>
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
