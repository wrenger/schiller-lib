<script lang="ts">
	import { _, date } from 'svelte-i18n';
	import { onMount } from 'svelte';
	import { DateTime } from 'luxon';
	import api from '$lib/api';

	let overdoneBooks: Promise<[api.Book, api.User][]>;

	onMount(() => {
		overdoneBooks = api.overdues();
	});
</script>

<svelte:head>
	<title>{$_('.book.overdues')}</title>
	<meta name="description" content={$_('.book.overdues')} />
</svelte:head>

<div class="w-full h-full text-token md:card dark:bg-surface-800 bg-surface-100 p-2">
	<div class="p-2">
		<span class="flex">
			<span class="flex-auto font-bold">{$_('.book.title')}</span>
			<span class="font-bold">{$_('.book.period.date')} / {$_('.book.period.days')}</span>
		</span>
	</div>
	<hr />
	<nav class="list-nav overflow-y-scroll max-h">
		<ul>
			{#await overdoneBooks then data}
				{#if data}
					{#each data as [book, user] (book.id)}
						<li>
							<a
								href={`books?${new URLSearchParams({ search: book.id })}`}
								style="border-radius: 12px;"
								class="!p-2"
							>
								<span class="flex-auto truncate w-[200px]"
									><p>{book.title}</p>
									<small class="text-sm opacity-50"
										>{$_('.book.overdone.by.short', {
											values: { '0': `${user.forename} ${user.surname}` }
										})}</small
									></span
								>
								<span class="text-end truncate">
									<p>
										{$_('.book.period', {
											values: {
												'0': DateTime.fromISO(book.deadline ?? '').toLocaleString(),
												'1': parseInt(
													(-DateTime.fromISO(book.deadline ?? '').diff(DateTime.now(), 'days')
														.days).toLocaleString()
												)
											}
										})}
									</p>
								</span>
							</a>
						</li>
					{:else}
						<li>
							<span class="opacity-50 flex-auto"><dd class="p-2">{$_('.error.none')}</dd></span>
						</li>
					{/each}
				{/if}
			{/await}
		</ul>
	</nav>
	<div class="p-2 pt-0"></div>
</div>

<style>
	.max-h {
		--border-height: 50px;
		height: calc(100% - var(--border-height));
	}
</style>
