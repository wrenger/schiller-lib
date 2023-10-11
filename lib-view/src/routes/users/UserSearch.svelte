<script lang="ts" context="module">
	export class UserParams {
		input!: string;
		permission!: null | boolean;
	}
</script>

<script lang="ts">
	import { _ } from "svelte-i18n";

	export let params: UserParams = {
		input: "",
		permission: null
	};

	let input: string;
	let permission!: null | boolean;
</script>

<div class="input-group mb-2">
	<input
		type="text"
		class="form-control"
		placeholder={$_(".search.user.entry")}
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
		title="Advanced Params"
	>
		<i class="bi bi-sliders" />
	</button>
	<ul class="dropdown-menu dropdown-menu-end" id="select-dropdown">
		<li>
			<h6 class="dropdown-header">{$_(".user.permission")}</h6>
		</li>
		<form class="px-3 py-1" action="javascript:handleAdvanced()">
			<div class="mb-2">
				<select
					id="select"
					class="form-select"
					aria-label={$_(".search.advanced")}
					bind:value={permission}
					on:change={() => (params.permission = permission)}
				>
					<option value={null} selected>{$_(".action.select")}</option>
					<option value={true}>{$_(".user.may-borrow")}</option>
					<option value={false}>{$_(".user.may-not-borrow")}</option>
				</select>
			</div>
		</form>
	</ul>
</div>

<style>
	.hide-arrow::after {
		display: none !important;
	}
</style>
