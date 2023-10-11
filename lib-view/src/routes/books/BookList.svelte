<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { Book } from "./BookView.svelte";

	export let items: Book[];
	export let active: Book | null;
	export let isNew: boolean;

	$: if (active || !active) {
		active = items.find((item) => active && item.id == active.id) || null;
		if (active) {
			const element = document.getElementById(active.id);
			if (element) {
				element.scrollIntoView({ behavior: "smooth", block: "nearest" });
			}
		}
	}
</script>

<div class="card list">
	<div class="card-header d-flex justify-content-between">
		{$_(".book.title")} / {$_(".book.authors")}
		<span>{$_(".book.id")} / {$_(".book.state")}</span>
	</div>
	<ul class="list-group list-group-flush list-body">
		{#each items as item}
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
	</ul>
	<div class="card-footer d-flex justify-content-between align-items-center">
		{$_(".search.results", { values: { 0: items.length } })}
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
