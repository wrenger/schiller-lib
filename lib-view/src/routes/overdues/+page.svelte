<script lang="ts">
	import { _ } from "svelte-i18n";
	import { onMount } from "svelte";
	import { DateTime } from "luxon";
	import api from "$lib/api";

	let overdoneBooks: Promise<[api.Book, api.User][]>;

	onMount(() => {
		overdoneBooks = api.overdues();
	});
</script>

<svelte:head>
	<title>{$_(".book.overdues")}</title>
	<meta name="description" content={$_(".book.overdues")} />
</svelte:head>

<div class="card h-100">
	<div class="card-header d-flex justify-content-between">
		{$_(".book.title")}
		<div class="d-flex align-items-center">
			<span>{$_(".book.period.date")} / {$_(".book.period.days")}</span>
		</div>
	</div>
	<ul class="list-group list-group-flush full">
		{#await overdoneBooks}
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
					<a
						class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
						href={`books?${new URLSearchParams({ search: book.id })}`}
					>
						<div class="d-flex flex-column">
							<p class="mb-0 text-truncate h">{book.title}</p>
							<small class="mb-0 text-muted text-truncate h">
								{$_(".book.overdone.by.short", {
									values: { "0": `${user.forename} ${user.surname}` }
								})}
							</small>
						</div>
						<div class="d-flex flex-column">
							<p class="mb-0 text-truncate h">
								{$_(".book.period", {
									values: {
										"0": DateTime.fromISO(book.deadline ? book.deadline : "").toLocaleString(),
										"1": parseInt(
											(-DateTime.fromISO(book.deadline ? book.deadline : "").diff(
												DateTime.now(),
												"days"
											).days).toLocaleString()
										)
									}
								})}
							</p>
						</div>
					</a>
				{:else}
					<li class="list-group-item disabled">{$_(".error.none")}</li>
				{/each}
			{/if}
		{/await}
	</ul>
</div>

<style>
	.full {
		overflow: scroll;
		height: calc(100% - 41px);
	}
</style>
