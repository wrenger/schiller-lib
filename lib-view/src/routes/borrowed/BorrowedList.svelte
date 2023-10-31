<script lang="ts">
	export let promise: Promise<any>;
</script>

<ul class="list-group list-group-flush">
	{#await promise}
		<li class="list-group-item">
			<div class="d-flex justify-content-center">
				<div class="spinner-grow" role="status">
					<span class="visually-hidden">Loading...</span>
				</div>
			</div>
		</li>
	{:then data}
		{#if data}
			{#each data as [book, user] (book.id)}
				<slot item={[book, user]} class="list-group-item list-group-item-action" />
			{/each}
		{/if}
	{/await}
</ul>
