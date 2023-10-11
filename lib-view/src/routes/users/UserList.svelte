<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { User } from "./UserView.svelte";

	export let items: User[];
	export let active: User | null;
	export let isNew: boolean;

	$: if (active || !active) {
		active = items.find((item) => active && item.account == active.account) || null;
		if (active) {
			const element = document.getElementById(active.account);
			if (element) {
				element.scrollIntoView({ behavior: "smooth", block: "nearest" });
			}
		}
	}
</script>

<div class="card list">
	<div class="card-header d-flex justify-content-between align-items-center">
		{$_(".user.name")} / {$_(".user.account")}
		<span>{$_(".user.role")} </span>
	</div>
	<ul class="list-group list-group-flush list-body">
		{#each items as item}
			<button
				class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
				class:active={item === active}
				id={item.account}
				on:click={() => {
					active = item;
				}}
			>
				<div class="d-flex flex-column">
					<p class="mb-0 text-truncate">{item.forename} {item.surname}</p>
					<small class="text-muted text-truncate">{item.account}</small>
				</div>
				<div class="d-flex flex-column align-items-end">
					<p class="mb-0 text-truncate">{item.role}</p>
				</div>
			</button>
		{/each}
	</ul>
	<div class="card-footer d-flex justify-content-between align-items-center">
		{$_(".search.results", { values: { 0: items.length } })}
		<button
			class="btn btn-outline-primary {isNew ? 'active' : ''}"
			type="button"
			title={$_(".user.new")}
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
