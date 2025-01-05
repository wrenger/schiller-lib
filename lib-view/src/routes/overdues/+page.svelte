<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Layout from '../Layout.svelte';
	import { DateTime } from 'luxon';
	import api from '$lib/api';
	import { handle_result } from '$lib';

	async function loadOverdues() {
		return handle_result(await api.lending_overdues());
	}
</script>

<svelte:head>
	<title>{$_('.book.overdues')}</title>
	<meta name="description" content={$_('.book.overdues')} />
</svelte:head>

<Layout display={false}>
	<svelte:fragment slot="list-nav">
		<div class="flex h-full items-center px-4">
			<h1 class="text-xl font-bold">{$_('.book.overdues')}</h1>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="list">
		<div class="h-full overflow-y-scroll">
			<div class="flex flex-col gap-2 p-4">
				{#await loadOverdues() then data}
					{#each data as { book, user } (book.id)}
						<a
							class={'flex w-full flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all hover:bg-accent'}
							href={`books?${new URLSearchParams({ search: book.id })}`}
						>
							<div class="grid w-full grid-cols-[1fr_auto] gap-1">
								<div class="w-full overflow-hidden">
									<div class="truncate font-semibold">{book.title}</div>
									<div class={'ml-auto truncate text-xs'}>
										{$_('.book.borrowed.by.short', {
											values: { '0': `${user.forename} ${user.surname}` }
										})}
									</div>
								</div>
								<div class="flex items-center text-xs font-medium">
									{$_('.book.period', {
										values: {
											'0': DateTime.fromISO(book.borrower?.deadline ?? '').toLocaleString(),
											'1': Math.round(
												-DateTime.fromISO(book.borrower?.deadline ?? '').diff(
													DateTime.now(),
													'days'
												).days
											)
										}
									})}
								</div>
							</div>
						</a>
					{:else}
						<div class="text-nowrap text-muted-foreground">{$_('.error.none')}</div>
					{/each}
				{/await}
			</div>
		</div>
	</svelte:fragment>
</Layout>
