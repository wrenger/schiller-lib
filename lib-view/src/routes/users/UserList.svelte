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
					<p class="mb-0">{item.forename} {item.surname}</p>
					<small class="text-muted">{item.account}</small>
				</div>
				<div class="d-flex flex-column align-items-end">
					<p class="mb-0">{item.role}</p>
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
		height: calc(var(--box) - 45px);
	}
	.list-body {
		overflow-y: scroll;
		flex: 1;
	}
</style>
