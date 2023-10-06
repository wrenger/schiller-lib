<section class="mb-2">
	<div class="input-group">
		<input
			type="text"
			class="form-control"
			placeholder="Keyword (E.g. Author, Title, Id, User)"
			id="search"
			on:keypress={(e) => {
				if (e.key == "Enter") console.log("Initiated Search");
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
				<h6 class="dropdown-header">Category</h6>
			</li>
			<form class="px-3 py-1" action="javascript:handleAdvanced()">
				<div class="mb-2">
					<select
						id="select"
						class="form-select"
						aria-label="Advanced Select"
						on:change={() => console.log("Initiated Search")}
					>
						<option value={null} selected>All</option>
						<option value={1}>1</option>
						<option value={2}>2</option>
						<option value={3}>3</option>
						<option value={4}>4</option>
						<option value={5}>5</option>
						<option value={6}>6</option>
						<option value={7}>7</option>
						<option value={8}>8</option>
						<option value={9}>9</option>
						<option value={10}>10</option>
						<option value={11}>11</option>
						<option value={12}>12</option>
						<option value={13}>13</option>
						<option value={14}>14</option>
					</select>
				</div>
			</form>
			<li>
				<h6 class="dropdown-header">Status</h6>
			</li>
			<form class="px-3 py-1" action="javascript:handleAdvanced()">
				<div class="mb-2">
					<select
						id="select"
						class="form-select"
						aria-label="Advanced Select"
						on:change={() => console.log("Initiated Search")}
					>
						<option value={null} selected>All</option>
						<option value={1}>Borrowable</option>
						<option value={2}>Not Borrowable</option>
						<option value={3}>Borrowed</option>
					</select>
				</div>
			</form>
		</ul>
	</div>
</section>

<style>
	.hide-arrow::after {
		display: none !important;
	}

	.input-group {
		width: 50%;
	}
	@media (max-width: 768px) {
		.input-group {
			width: 100%;
		}
	}
</style>
