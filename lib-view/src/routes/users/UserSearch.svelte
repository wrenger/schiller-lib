<script lang="ts" context="module">
	export class UserParams {
		input!: string;
		permission!: null | boolean;
	}
</script>

<script lang="ts">
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
		placeholder="Keyword (E.g. Name, Account, Role)"
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
			<h6 class="dropdown-header">Borrow Permission</h6>
		</li>
		<form class="px-3 py-1" action="javascript:handleAdvanced()">
			<div class="mb-2">
				<select
					id="select"
					class="form-select"
					aria-label="Advanced Select"
					bind:value={permission}
					on:change={() => (params.permission = permission)}
				>
					<option value={null} selected>All</option>
					<option value={true}>Can Borrow</option>
					<option value={false}>Cannot Borrow</option>
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
