<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';

	import { DateTime } from 'luxon';
	import CategorySelect from '../../components/basic/CategorySelect.svelte';
	import Spinner from '../../components/basic/Spinner.svelte';

	export let editable: boolean = false;

	let id = '';
	let isbn = '';
	let title = '';
	let publisher = '';
	let authors = '';
	let costs = 0;
	let year = DateTime.now().year;
	let category = '';
	let note = '';
	let borrowable = true;
	let borrower: api.Borrower | undefined = undefined;
	let deadline: DateTime | undefined = undefined;
	let reservation: string | undefined = undefined;

	let bookIdResponse: Promise<any>;
	let isbnResponse: Promise<any>;

	function defaultBook(): api.Book {
		return {
			id: '',
			isbn: '',
			title: '',
			publisher: '',
			authors: '',
			costs: 0,
			year: DateTime.now().year,
			category: '',
			borrowable: true
		};
	}

	export function setBook(b: api.Book | null) {
		b ??= defaultBook();

		id = b.id;
		isbn = b.isbn;
		title = b.title;
		publisher = b.publisher;
		authors = b.authors;
		costs = b.costs;
		year = b.year;
		category = b.category;
		note = b.note ?? '';
		borrowable = b.borrowable;
		borrower = b.borrower;
		reservation = b.reservation;
	}

	export function getBook(): api.Book {
		return {
			id,
			isbn,
			title,
			publisher,
			authors,
			costs,
			year,
			category,
			note: note || undefined,
			borrowable,
			borrower: borrower,
			reservation: reservation
		};
	}
</script>

<label class="label">
	<span>{$_('.book.title')}</span>
	<input
		class="input"
		type="text"
		placeholder={$_('.book.title')}
		disabled={!editable}
		bind:value={title}
	/>
</label>
<div class="w-full grid grid-cols-2 gap-4">
	<label class="label">
		<span>{$_('.book.id')}</span>
		<div class="input-group grid-cols-[1fr_auto] mb-2 {!editable ? 'disabled' : ''}">
			<input
				class="input"
				type="text"
				placeholder={$_('.book.id')}
				disabled={!editable}
				bind:value={id}
			/>
			<button
				class="variant-soft"
				title={$_('.book.id.action')}
				disabled={!editable}
				on:click={async () => {
					bookIdResponse = api.book_id(getBook());
					console.log(bookIdResponse);
					id = await bookIdResponse;
				}}
			>
				<Spinner response={bookIdResponse} />
				<i class="fa-solid fa-rotate"></i>
			</button>
		</div>
	</label>

	<label class="label">
		<span>{$_('.book.isbn')}</span>
		<div class="input-group grid-cols-[1fr_auto] mb-2 {!editable ? 'disabled' : ''}">
			<input
				class="input"
				type="text"
				placeholder={$_('.book.isbn')}
				disabled={!editable}
				bind:value={isbn}
			/>
			<button
				class="variant-soft"
				type="button"
				title={$_('.book.request')}
				disabled={!editable}
				on:click={async () => {
					isbnResponse = api.book_fetch(isbn);
					let data = await isbnResponse;
					title = data.title ?? '';
					publisher = data.publisher ?? '';
					authors = Array.isArray(data.authors) ? data.authors.join(', ') : data.authors ?? '';
					costs = data.costs ?? 0;
				}}
			>
				<Spinner response={isbnResponse} />
				<i class="fa-solid fa-download"></i>
			</button>
		</div>
	</label>
</div>

<label class="label">
	<span>{$_('.book.authors')}</span>
	<input
		class="input"
		type="text"
		placeholder={$_('.book.authors')}
		disabled={!editable}
		bind:value={authors}
	/>
</label>

<div class="w-full grid grid-cols-2 gap-4">
	<label class="label">
		<span>{$_('.book.publisher')}</span>
		<input
			class="input"
			type="text"
			placeholder={$_('.book.publisher')}
			disabled={!editable}
			bind:value={publisher}
		/>
	</label>
	<label class="label">
		<span>{$_('.book.costs')}</span>
		<input
			class="input"
			type="number"
			placeholder={$_('.book.costs')}
			disabled={!editable}
			bind:value={costs}
		/>
	</label>

	<label class="label">
		<span>{$_('.book.year')}</span>
		<input
			class="input"
			type="number"
			placeholder={$_('.book.year')}
			disabled={!editable}
			bind:value={year}
		/>
	</label>

	<CategorySelect bind:value={category} disabled={!editable} label={$_('.category')} />
</div>

<label class="label">
	<span>{$_('.book.note')}</span>
	<textarea
		class="textarea"
		rows="3"
		placeholder={$_('.book.note')}
		disabled={!editable}
		bind:value={note}
	/>
</label>

<label class="flex items-center space-x-2">
	<input class="checkbox" type="checkbox" bind:checked={borrowable} disabled={!editable} />
	<p>{$_('.book.borrowable')}</p>
</label>

{#if !editable}
	{#if borrower && deadline}
		<aside class="alert variant-glass-surface mt-1">
			{$_('.book.borrowed.by', {
				values: {
					'0': borrower?.user ?? '',
					'1': borrower?.deadline.toLocaleString()
				}
			})}
		</aside>
		{#if reservation}
			<aside class="alert variant-glass-surface mt-1">
				{$_('.book.reserved.by', { values: { '0': reservation } })}
			</aside>
		{/if}
	{:else if reservation}
		<aside class="alert variant-glass-surface mt-1">
			{$_('.book.reserved.by', { values: { '0': reservation } })}
		</aside>
	{:else if borrowable}
		<aside class="alert variant-glass-surface mt-1">
			{$_('.book.available.long')}
		</aside>
	{:else}
		<aside class="alert variant-glass-surface mt-1">
			{$_('.book.not-borrowable')}
		</aside>
	{/if}
{/if}
