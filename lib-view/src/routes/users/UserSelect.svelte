<script lang="ts">
	import { _ } from "svelte-i18n";
	import Request from "../../components/basic/Request.svelte";
	import type { User } from "./UserView.svelte";
	export let value = "";
	export let label = "";
	export let placeholder = "";
	export let editable: boolean = true;

	let items: Promise<User[]> | never[] = [];
	let r: Request;
</script>

<Request bind:this={r} />

<label for="select-{label}" class="form-label">{label}</label>
<div class="input-group mb-3" id="select-{label}">
	<button
		id="select-button"
		class="btn btn-outline-secondary dropdown-toggle hide-arrow"
		type="button"
		data-bs-toggle="dropdown"
		aria-expanded="false"
		title={$_(".action.select")}
		disabled={!editable}
		on:click={() => (items = r.request(`api/user?query=${value}&limit=10`, "GET", null))}
	>
		<i class="bi bi-search" />
	</button>
	<ul id="select-dropdown" class="dropdown-menu select" hidden={!editable}>
		{#await items}
			<li class="dropdown-item">
				<div class="d-flex justify-content-center">
					<div class="spinner-grow" role="status">
						<span class="visually-hidden">Loading...</span>
					</div>
				</div>
			</li>
		{:then data}
			{#each data as entry}
				<button
					class="dropdown-item"
					on:click={() => {
						value = entry.account;
					}}>{entry.account}</button
				>
			{:else}
				<button class="dropdown-item" disabled>{$_(".error.none")}</button>
			{/each}
		{/await}
	</ul>
	<input
		type="text"
		class="form-control"
		{placeholder}
		aria-label={placeholder}
		readonly={!editable}
		bind:value
	/>
</div>

<style>
	.hide-arrow::after {
		display: none !important;
	}
	.select {
		overflow: scroll;
		max-height: 200%;
	}
</style>
