<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type api from '$lib/api';

	type T = $$Generic<{}>;

	const CHUNK_SIZE: number = 200;

	export let rowHeight: number;
	export let scrollClass: string = '';
	export let active: T | null;
	export let load: (offset: number, limit: number) => Promise<api.Limited<T>>;
	export let onLoad: (totalCount: number) => void = () => {};
	export let key: (t: T) => string;

	// we cannot use promises here, as we resize the list and the references get out of sync
	let chunks: (T[] | 'loading' | null)[] = [];
	let firstChunk = -1;
	let lastChunk = -1;
	let totalCount: number = 0;

	let scroller: HTMLDivElement | undefined;

	export async function reload() {
		updateChunks(true);
	}

	async function loadChunk(i: number): Promise<void> {
		let { rows, total } = await load(i * CHUNK_SIZE, CHUNK_SIZE);
		onLoad(total);

		// Grow list
		if (totalCount !== total) {
			totalCount = total;
			for (let j = chunks.length; j < totalCount / CHUNK_SIZE; j++) {
				chunks.push(null);
			}
		}

		if (chunks[i] != null) chunks[i] = rows;

		// Update active
		if (active != null) {
			let a = key(active);
			let row = rows.find((v) => key(v) == a);
			if (row != null) {
				active = row;
			}
		} else {
			active = rows[0];
		}

		// trigger update
		chunks = chunks;
	}

	async function updateChunks(needsReload = false) {
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
					if (chunks[i] == null) chunks[i] = 'loading';
					await loadChunk(i);
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
	}
</script>

<div
	bind:this={scroller}
	on:scroll={() => updateChunks()}
	class="h-full overflow-y-scroll px-4 {scrollClass}"
>
	<div style="height: {rowHeight * totalCount}px; position: relative;">
		{#each chunks as chunk, i (i)}
			<!--  on selected: bg-muted  -->
			{#if chunk != null && chunk != 'loading'}
				<div
					class="flex flex-col gap-2"
					style="position: absolute; top: {i * rowHeight * CHUNK_SIZE}px; left: 0; right: 0;"
				>
					{#each chunk as item (key(item))}
						<slot name="item" {item} />
					{/each}
				</div>
			{/if}
		{:else}
			<div class="text-nowrap text-muted-foreground">{$_('.error.none')}</div>
		{/each}
	</div>
</div>
