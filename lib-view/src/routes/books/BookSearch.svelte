<script lang="ts" context="module">
	export class BookParams {
		input: string;
		category: null | string;
		status!: "None" | "Borrowable" | "NotBorrowable" | "Borrowed" | "Reserved";

		constructor(
			params: {
				input?: string;
				category?: null | string;
				status?: "None" | "Borrowable" | "NotBorrowable" | "Borrowed" | "Reserved";
			} = {}
		) {
			this.input = params.input || "";
			this.category = params.category || null;
			this.status = params.status || "None";
		}
	}
</script>

<script lang="ts">
	import { _ } from "svelte-i18n";
	import { page } from "$app/stores";
	import CategorySelect from "../../components/basic/CategorySelect.svelte";

	export let params: BookParams = new BookParams();

	let input: string = "";
	let category: null | string = null; //temporary - todo: add categories
	let status: "None" | "Borrowable" | "NotBorrowable" | "Borrowed" | "Reserved" = "None";

	input = $page.url.searchParams.get("i") || "";
	params.input = input;

	let timer: NodeJS.Timeout | null = null;

	function handleInputDelayed() {
		if (timer) {
			clearTimeout(timer);
		}
		timer = setTimeout(() => {
			params.input = input;
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
						bind:value={status}
						on:change={() => (params.status = status)}
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
