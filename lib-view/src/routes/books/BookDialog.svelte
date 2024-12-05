<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { handle_result, onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { DateTime } from 'luxon';
	import { Label } from '$lib/components/ui/label';
	import Input from '$lib/components/ui/input/input.svelte';
	import * as Select from '$lib/components/ui/select';
	import { categories } from '$lib/store';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Download, RefreshCcw } from 'lucide-svelte';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let id = '';
	let isbn = '';
	let title = '';
	let publisher = '';
	let authors = '';
	let costs = 0;
	let year = DateTime.now().year;
	let category: api.Category | null = null;
	let note = '';
	let borrowable = true;
	let borrower: api.Borrower | undefined = undefined;
	let reservation: string | undefined = undefined;

	function setBook() {
		if (book) {
			id = book.id;
			isbn = book.isbn;
			title = book.title;
			publisher = book.publisher;
			authors = book.authors;
			costs = book.costs;
			year = book.year;
			category = $categories?.find((c) => c.id == book.category) ?? null;
			note = book.note ?? '';
			borrowable = book.borrowable;
			borrower = book.borrower;
			reservation = book.reservation;
		} else {
			id = '';
			isbn = '';
			title = '';
			publisher = '';
			authors = '';
			costs = 0;
			year = DateTime.now().year;
			category = null;
			note = '';
			borrowable = true;
			borrower = undefined;
			reservation = undefined;
		}
	}

	function getBook(): api.Book {
		return {
			id,
			isbn,
			title,
			publisher,
			authors,
			costs,
			year,
			category: category?.id ?? '',
			note: note ?? undefined,
			borrowable,
			borrower: borrower,
			reservation: reservation
		};
	}

	let open = false;
	let idResponse: Promise<api.Result<string>>;
	let isbnResponse: Promise<api.Result<api.BookData>>;

	$: if (open) setBook();

	let addResponse: Promise<any>;
	async function add() {
		let book = handle_result(await api.book_add(getBook()));
		open = false;
		onChange(book);
	}

	let editResponse: Promise<any>;
	async function edit() {
		if (book) {
			let newBook = handle_result(await api.book_update(book.id, getBook()));
			open = false;
			onChange(newBook);
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={(value) => (open = value)} {onOutsideClick}>
	<Dialog.Trigger asChild let:builder={dialog}>
		<slot {dialog} />
	</Dialog.Trigger>
	<Dialog.Content class="max-h-full overflow-y-scroll">
		<Dialog.Header>
			<Dialog.Title>
				{#if book}
					{$_('.action.edit')}
				{:else}
					{$_('.action.add')}
				{/if}
			</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4">
			<div class="flex w-full flex-col gap-1.5">
				<Label for="title" class="text-left">{$_('.book.title')}</Label>
				<Input id="title" placeholder={$_('.book.title')} bind:value={title} />
			</div>
			<div class="grid grid-cols-2 space-x-1">
				<div class="flex w-full flex-col gap-1.5">
					<Label for="id" class="text-left">{$_('.book.id')}</Label>
					<div class="relative">
						<Button
							size="icon"
							variant="ghost"
							title={$_('.book.id.action')}
							class="absolute left-2 top-2.5 h-5 w-5 p-[2px] text-muted-foreground"
							on:click={async () => {
								idResponse = api.book_generate_id(getBook());
								id = handle_result(await idResponse);
							}}
						>
							<Spinner response={idResponse} spinnerClass="size-5 !mr-0">
								<RefreshCcw class="size-5" />
							</Spinner>
						</Button>
						<Input id="id" class="pl-8" placeholder={$_('.book.id')} bind:value={id} />
					</div>
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="isbn" class="text-left">{$_('.book.isbn')}</Label>
					<div class="relative">
						<Button
							size="icon"
							variant="ghost"
							title={$_('.book.request')}
							class="absolute left-2 top-2.5 h-5 w-5 p-[2px] text-muted-foreground"
							on:click={async () => {
								isbnResponse = api.book_fetch_data(isbn);
								let data = handle_result(await isbnResponse);
								title = data.title ?? '';
								publisher = data.publisher ?? '';
								authors = Array.isArray(data.authors)
									? data.authors.join(', ')
									: (data.authors ?? '');
								costs = data.costs ?? 0;
							}}
						>
							<Spinner response={isbnResponse} spinnerClass="size-5 !mr-0">
								<Download class="size-5" />
							</Spinner>
						</Button>
						<Input id="isbn" class="pl-8" placeholder={$_('.book.isbn')} bind:value={isbn} />
					</div>
				</div>
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="authors" class="text-left">{$_('.book.authors')}</Label>
				<Input id="authors" placeholder={$_('.book.authors.def')} bind:value={authors} />
			</div>
			<div class="grid grid-cols-2 space-x-1">
				<div class="flex w-full flex-col gap-1.5">
					<Label for="publisher" class="text-left">{$_('.book.publisher')}</Label>
					<Input id="publisher" placeholder={$_('.book.publisher')} bind:value={publisher} />
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="costs" class="text-left">{$_('.book.costs')}</Label>
					<Input id="costs" type="number" placeholder={$_('.book.costs')} bind:value={costs} />
				</div>
			</div>
			<div class="grid grid-cols-2 space-x-1">
				<div class="flex w-full flex-col gap-1.5">
					<Label for="year" class="text-left">{$_('.book.year')}</Label>
					<Input id="year" type="number" placeholder={$_('.book.year')} bind:value={year} />
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="category" class="text-left">{$_('.category')}</Label>
					<Select.Root
						selected={{
							value: category?.id,
							label: category
								? `${category?.id} - ${category?.name} - ${category?.section}`
								: $_('.action.select')
						}}
					>
						<Select.Trigger class="w-full" id="category">
							<Select.Value placeholder={$_('.category')} />
						</Select.Trigger>
						<Select.Content class="text-left">
							<div class="max-h-72 overflow-y-scroll">
								{#each $categories as a_category}
									<Select.Item on:click={() => (category = a_category)} value={a_category.id}
										>{a_category.id} - {a_category.name} - {a_category.section}</Select.Item
									>
								{/each}
							</div>
						</Select.Content>
					</Select.Root>
				</div>
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="note" class="text-left">{$_('.book.note')}</Label>
				<Textarea id="note" placeholder={$_('.book.note')} bind:value={note} />
			</div>
			<div class="flex items-center space-x-2">
				<Checkbox id="borrowable" bind:checked={borrowable} aria-labelledby="borrowable-label" />
				<Label
					id="borrowable-label"
					for="borrowable"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					{$_('.book.borrowable')}
				</Label>
			</div>
		</div>
		<Dialog.Footer>
			{#if book}
				<Button on:click={() => (editResponse = edit())}>
					<Spinner response={editResponse} />
					{$_('.action.apply')}
				</Button>
			{:else}
				<Button on:click={() => (addResponse = add())}>
					<Spinner response={addResponse} />
					{$_('.action.apply')}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
