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
					<p class="mb-0">{item.title}</p>
					<small class="text-muted">{item.authors.join(", ")}</small>
				</div>
				<div class="d-flex flex-column align-items-end">
					<small class="text-muted">{item.id}</small>
					<p class="mb-0">
						{item.borrower || item.reservation
							? `${$_(".book.unavailable")}`
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
			on:click={() => (isNew = true)}
			><svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentColor"
				class="bi bi-plus-lg"
				viewBox="0 0 16 16"
			>
				<path
					fill-rule="evenodd"
					d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2Z"
				/>
			</svg></button
		>
	</div>
</div>

<style>
	.list-group-item-action {
		cursor: pointer;
	}
	.list {
		height: calc(var(--list-height) - 45px);
	}
	.list-body {
		overflow-y: scroll;
		flex: 1;
	}
</style>
