<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type api from '$lib/api';
	import * as Alert from '$lib/components/ui/alert';
	import { Badge } from '$lib/components/ui/badge';
	import { BookDashed } from 'lucide-svelte';
	import { DateTime } from 'luxon';
	import { categories } from '$lib/store';

	export let book: api.Book;

	$: category = $categories?.find((c) => c.id == book.category);

	function displayInformation(info: string | number | undefined) {
		return info ? info : $_('.action.empty');
	}
</script>

<div class="h-full w-full space-y-4 overflow-y-scroll p-4">
	<div class="border-b pb-3">
		<h2 class="text-xl font-semibold">{book.title}</h2>
		<p class="text-md">
			<span class="text-sm text-muted-foreground">{$_('.book.id')}:</span> {displayInformation(book.id)}
		</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.book.isbn')}</h3>
		<p class="text-md font-medium">{displayInformation(book.isbn)}</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.book.authors')}</h3>
		<p class="text-md font-medium">{displayInformation(book.authors)}</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.book.publisher')}</h3>
		<p class="text-md font-medium">{displayInformation(book.publisher)}</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.book.costs')}</h3>
		<p class="text-md font-medium">{displayInformation(book.costs)}</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.book.year')}</h3>
		<p class="text-md font-medium">{displayInformation(book.year)}</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.category')}</h3>
		<p class="text-md font-medium">
			{category
				? `${category.id} - ${category.name} - ${category.section}`
				: displayInformation(book.category)}
		</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.book.note')}</h3>
		<p class="text-md font-medium">{displayInformation(book.note)}</p>
	</div>
	<div class="flex items-center space-x-2">
		<span class="text-md font-medium">{$_('.book.borrowable')}:</span>
		{#if book.borrowable}
			<Badge>{$_('.action.yes')}</Badge>
		{:else}
			<Badge>{$_('.action.no')}</Badge>
		{/if}
	</div>
	{#if book.borrower}
		<Alert.Root>
			<BookDashed class="h-4 w-4" />
			<Alert.Title>{$_('.book.state')}</Alert.Title>
			<Alert.Description>
				{$_('.book.borrowed.by', {
					values: {
						'0': book.borrower.user,
						'1': DateTime.fromISO(book.borrower.deadline).toLocaleString()
					}
				})}
				{#if book.reservation}
					<div class="pt-1">{$_('.book.reserved.by', { values: { '0': book.reservation } })}</div>
				{/if}
			</Alert.Description>
		</Alert.Root>
	{:else if book.reservation}
		<Alert.Root>
			<BookDashed class="h-4 w-4" />
			<Alert.Title>{$_('.book.state')}</Alert.Title>
			<Alert.Description
				>{$_('.book.reserved.by', { values: { '0': book.reservation } })}</Alert.Description
			>
		</Alert.Root>
	{:else if book.borrowable}
		<Alert.Root>
			<BookDashed class="h-4 w-4" />
			<Alert.Title>{$_('.book.state')}</Alert.Title>
			<Alert.Description>{$_('.book.available.long')}</Alert.Description>
		</Alert.Root>
	{:else}
		<Alert.Root>
			<BookDashed class="h-4 w-4" />
			<Alert.Title>{$_('.book.state')}</Alert.Title>
			<Alert.Description>{$_('.book.not-borrowable')}</Alert.Description>
		</Alert.Root>
	{/if}
</div>
