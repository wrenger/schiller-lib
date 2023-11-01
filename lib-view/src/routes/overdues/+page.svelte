<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { Book } from "../books/BookView.svelte";
	import { onMount } from "svelte";
	import Request from "../../components/basic/Request.svelte";
	import type { User } from "../users/UserView.svelte";
	import { DateTime } from "luxon";

	let overdoneBooks: Promise<[Book, User][]>;
	let r: Request;

	onMount(() => {
		overdoneBooks = r.request("api/overdues", "GET", null);
	});
</script>

<svelte:head>
	<title>{$_(".book.overdues")}</title>
	<meta name="description" content={$_(".book.overdues")} />
</svelte:head>

<Request bind:this={r} />

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
						href={`books?i=${book.id}`}
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
		height: calc(100% - 51px);
	}
</style>
