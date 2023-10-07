<script lang="ts" context="module">
	export class Book {
		id!: string;
		isbn!: number;
		title!: string;
		publisher!: string;
		authors!: string[];
		costs!: number;
		year!: number;
		category!: string; //temporary - todo: add categories
		note?: string;
		borrowed!: boolean;
		borrowable!: boolean;
	}
</script>

<script lang="ts">
	export let items: Book[];

	let entry: Book;
</script>

<div class="list">
	<div class="card">
		<div class="card-header d-flex justify-content-between">
			Title / Authors
			<span>Id / Status</span>
		</div>
		<ul class="list-group list-group-flush list-body">
			{#each items as item}
				<button
					class="list-group-item list-group-item-action d-flex justify-content-between"
					class:active={item === entry}
					on:click={() => {entry = item; console.log("Show:", entry)}}
				>
					<div class="d-flex flex-column">
						<p class="mb-0">{item.title}</p>
						<small class="text-muted">{item.authors.join(", ")}</small>
					</div>
					<div class="d-flex flex-column align-items-end">
						<small class="text-muted">{item.id}</small>
						<p class="mb-0">{item.borrowed ? "Borrowed" : "Available"}</p>
					</div>
				</button>
			{/each}
		</ul>
		<div class="card-footer d-flex justify-content-between align-items-center">
			{items.length} Results
			<button
				class="btn btn-primary"
				type="button"
				title="Add"
				on:click={() => console.log("Initiate Add")}
				><svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					fill="currentColor"
					class="bi bi-plus-lg"
					viewBox="0 0 16 16"
				>
					<path
						fill-rule="evenodd"
						d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2Z"
					/>
				</svg></button
			>
		</div>
	</div>
</div>

<style>
	.list-group-item-action {
		cursor: pointer;
	}
	.list {
		width: 50%;
	}
	.list-body {
		height: fit-content;
		overflow-y: scroll;
	}
	@media (max-width: 768px) {
		.list {
			width: 100%;
		}
	}
</style>
