<script lang="ts" context="module">
	export class BookParams {
		input!: string;
		category!: null | string; //temporary - todo: add categories
		status!: null | "borrowable" | "not-borrowable" | "borrowed";
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
	let status: null | "borrowable" | "not-borrowable" | "borrowed";

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
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentColor"
				class="bi bi-sliders"
				viewBox="0 0 16 16"
			>
				<path
					fill-rule="evenodd"
					d="M11.5 2a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zM9.05 3a2.5 2.5 0 0 1 4.9 0H16v1h-2.05a2.5 2.5 0 0 1-4.9 0H0V3h9.05zM4.5 7a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zM2.05 8a2.5 2.5 0 0 1 4.9 0H16v1H6.95a2.5 2.5 0 0 1-4.9 0H0V8h2.05zm9.45 4a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zm-2.45 1a2.5 2.5 0 0 1 4.9 0H16v1h-2.05a2.5 2.5 0 0 1-4.9 0H0v-1h9.05z"
				/>
			</svg>
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
						<option value={"borrowed"}>{$_(".book.unavailable")}</option>
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
