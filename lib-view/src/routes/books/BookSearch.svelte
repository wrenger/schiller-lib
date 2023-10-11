<script lang="ts" context="module">
	export class BookParams {
		input!: string;
		category!: null | string; //temporary - todo: add categories
		status!: null | "borrowable" | "not-borrowable" | "borrowed" | "reserved";
	}
</script>

<script lang="ts">
	import { _ } from "svelte-i18n";
	import { page } from "$app/stores";
	export let params: BookParams = {
		input: $page.url.searchParams.get("i") || "",
		category: null,
		status: null
	};

	let input: string;
	let category: null | string; //temporary - todo: add categories
	let status: null | "borrowable" | "not-borrowable" | "borrowed" | "reserved";

	input = $page.url.searchParams.get("i") || "";
</script>

<div class="d-flex">
	<div class="input-group mb-2">
		<input
			type="text"
			class="form-control"
			placeholder={$_(".search.book.entry")}
			id="search"
			bind:value={input}
			on:keypress={(e) => {
				if (e.key == "Enter") params.input = input;
			}}
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
		<ul class="dropdown-menu dropdown-menu-end" id="select-dropdown">
			<li>
				<h6 class="dropdown-header">{$_(".category")}</h6>
			</li>
			<form class="px-3 py-1" action="javascript:handleAdvanced()">
				<div class="mb-2">
					<select
						id="select"
						class="form-select"
						aria-label={$_(".search.advanced")}
						bind:value={category}
						on:change={() => (params.category = category)}
					>
						<option value={null} selected>{$_(".action.select")}</option>
						<option value={$_(".category.t1.id")}>{$_(".category.t1.name")}</option>
						<option value={$_(".category.t2.id")}>{$_(".category.t2.name")}</option>
					</select>
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
						<option value={null} selected>{$_(".action.select")}</option>
						<option value={"borrowable"}>{$_(".book.borrowable")}</option>
						<option value={"not-borrowable"}>{$_(".book.not-borrowable")}</option>
						<option value={"borrowed"}>{$_(".book.borrowed")}</option>
						<option value={"reserved"}>{$_(".book.reserved")}</option>
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
