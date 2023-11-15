<script lang="ts">
	import { _ } from "svelte-i18n";
	import { page } from "$app/stores";
	import type api from "$lib/api";
	import CategorySelect from "../../components/basic/CategorySelect.svelte";

	let input: string = $page.url.searchParams.get("search") || "";

	export let params: api.BookSearch = { query: input };

	let timer: NodeJS.Timeout | undefined = undefined;

	function handleInputDelayed() {
		clearTimeout(timer);
		timer = setTimeout(() => (params.query = input), 500);
	}
</script>

<div class="d-flex">
	<div class="input-group mb-2">
		<input
			type="text"
			class="form-control"
			placeholder={$_(".search.book.entry")}
			id="search"
			bind:value={input}
			on:input={handleInputDelayed}
		/>
		<button
			id="advanced"
			class="btn btn-outline-secondary dropdown-toggle hide-arrow"
			type="button"
			aria-expanded="false"
			data-bs-toggle="dropdown"
			data-bs-auto-close="outside"
			title={$_(".search.advanced")}
		>
			<i class="bi bi-sliders" />
		</button>
		<ul class="dropdown-menu dropdown-menu-end" id="select-dropdown" style="max-width: 250px;">
			<li>
				<h6 class="dropdown-header">{$_(".category")}</h6>
			</li>
			<form class="px-3 py-1" action="javascript:handleAdvanced()">
				<div class="mb-2">
					<CategorySelect bind:value={params.category} />
				</div>
			</form>
			<li>
				<h6 class="dropdown-header">{$_(".book.state")}</h6>
			</li>
			<form class="px-3 py-1" action="javascript:handleAdvanced()">
				<div class="mb-2">
					<select
						id="select"
						class="form-select"
						aria-label={$_(".search.advanced")}
						bind:value={params.state}
					>
						<option value={"None"} selected>{$_(".action.select")}</option>
						<option value={"Borrowable"}>{$_(".book.borrowable")}</option>
						<option value={"NotBorrowable"}>{$_(".book.not-borrowable")}</option>
						<option value={"Borrowed"}>{$_(".book.borrowed")}</option>
						<option value={"Reserved"}>{$_(".book.reserved")}</option>
					</select>
				</div>
			</form>
		</ul>
	</div>
</div>

<style>
	.hide-arrow::after {
		display: none !important;
	}
</style>
