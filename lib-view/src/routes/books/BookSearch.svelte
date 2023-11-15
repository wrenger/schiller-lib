<script lang="ts">
	import { _ } from "svelte-i18n";
	import { page } from "$app/stores";
	import CategorySelect from "../../components/basic/CategorySelect.svelte";
	import { goto } from "$app/navigation";
	import type api from "$lib/api";

	export let params: api.BookSearch = {};

	let input: string = "";
	let category: undefined | string = undefined;
	let state: "None" | "Borrowable" | "NotBorrowable" | "Borrowed" | "Reserved" = "None";

	input = $page.url.searchParams.get("i") || "";
	params.query = input;

	let timer: NodeJS.Timeout | null = null;

	function handleInputDelayed() {
		if (timer) {
			clearTimeout(timer);
		}
		timer = setTimeout(() => {
			params.query = input;
			goto(`/books${params.query.trim() ? `?i=${params.query}` : ""}`, {
				replaceState: false,
				keepFocus: true
			});
		}, 500);
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
					<CategorySelect bind:value={category} onChange={() => (params.category = category)} />
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
						bind:value={state}
						on:change={() => (params.state = state)}
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
