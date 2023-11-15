<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";
	export let value = "";
	export let label = "";
	export let placeholder = "";
	export let editable: boolean = true;

	let items: Promise<api.Limited<api.User>>;
</script>

<label for="select-{label}" class="form-label">{label}</label>
<div class="input-group mb-3" id="select-{label}">
	<input
		type="text"
		class="form-control"
		{placeholder}
		aria-label={placeholder}
		readonly={!editable}
		bind:value
	/>
	<button
		id="select-button"
		class="btn btn-outline-secondary dropdown-toggle hide-arrow"
		type="button"
		data-bs-toggle="dropdown"
		aria-expanded="false"
		title={$_(".action.select")}
		disabled={!editable}
		on:click={() => (items = api.user_search({ query: value, limit: 10 }))}
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
			{#if data && data.rows}
				{#each data.rows as entry}
					<button
						class="dropdown-item"
						on:click={() => {
							value = entry.account;
						}}>{entry.account}</button
					>
				{:else}
					<button class="dropdown-item" disabled>{$_(".error.none")}</button>
				{/each}
			{/if}
		{/await}
	</ul>
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
