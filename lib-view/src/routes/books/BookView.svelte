<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';

	import Spinner from '../../components/basic/Spinner.svelte';
	import BookDisplay from './BookDisplay.svelte';
	import { getModalStore, type ModalComponent, type ModalSettings } from '@skeletonlabs/skeleton';

	const modalStore = getModalStore();

	export var onChange: ((b: api.Book | null) => void) | undefined;

	export function display(b: api.Book) {
		state = {
			kind: State.Display,
			book: b,
			editing: false
		};
	}
	export function create() {
		state = { kind: State.Create };
	}

	// look what they need to mimic a fraction of our power
	enum State {
		Display,
		Create
	}
	interface Display {
		kind: State.Display;
		book: api.Book;
		editing: boolean;
	}
	interface Create {
		kind: State.Create;
	}

	let state: Display | Create = { kind: State.Create };

	let bookDisplay: BookDisplay;

	$: if (state.kind == State.Display) {
		if (bookDisplay) bookDisplay.setBook(state.book);
	}
	$: if (state.kind == State.Create) {
		if (bookDisplay) bookDisplay.setBook(null);
	}

	let addResponse: Promise<void>;
	async function add() {
		if (state.kind === State.Create) {
			let book = bookDisplay.getBook();
			await api.book_add(book);
			onChangeInner(book);
		}
	}

	let editResponse: Promise<void>;
	async function edit() {
		if (state.kind == State.Display) {
			let book = bookDisplay.getBook();
			await api.book_update(state.book.id, book);
			onChangeInner(book);
		}
	}

	let delResponse: Promise<void>;
	async function del() {
		if (state.kind == State.Display) {
			await api.book_delete(state.book.id);
			onChangeInner(null);
		}
	}

	let returnResponse: Promise<void>;
	async function returnBack() {
		if (state.kind == State.Display) {
			let book = await api.return_back(state.book.id);
			onChangeInner(book);
			if (book.reservation) mailModalOpen(book);
		}
	}

	let releaseResponse: Promise<void>;
	async function release() {
		if (state.kind == State.Display) {
			let book = await api.release(state.book.id);
			onChangeInner(book);
		}
	}

	function onChangeInner(book: api.Book | null) {
		if (book != null && state.kind === State.Display) {
			state.book = book;
			state.editing = false;
			bookDisplay.setBook(book);
		}
		if (onChange) onChange(book);
	}

	function lendModalOpen(borrower: string): void {
		const modal: ModalSettings = {
			type: 'component',
			component: 'lendModal',
			meta: {
				borrower: borrower,
				bookId: state.kind === State.Display ? state.book.id : '',
				onChange: onChangeInner
			}
		};
		modalStore.trigger(modal);
	}

	function reserveModalOpen(): void {
		const modal: ModalSettings = {
			type: 'component',
			component: 'reserveModal',
			meta: {
				reservation: state.kind === State.Display ? state.book.reservation : '',
				bookId: state.kind === State.Display ? state.book.id : '',
				onChange: onChangeInner
			}
		};
		modalStore.trigger(modal);
	}

	function mailModalOpen(book: api.Book): void {
		const modal: ModalSettings = {
			type: 'component',
			component: 'mailModal',
			meta: {
				book,
				onChange: onChangeInner
			}
		};
		modalStore.trigger(modal);
	}
</script>

<div
	class="w-full h-fit text-token md:card dark:bg-surface-800 bg-surface-100 p-2 space-y-2"
>
	<div class="flex p-2 pb-0">
		<span class="flex-auto">
			<button
				id="cancel"
				class="btn-icon variant-filled"
				type="button"
				aria-expanded="false"
				title={$_('.action.close')}
				on:click={() => onChangeInner(null)}
			>
				<i class="fa-solid fa-angle-left"></i>
			</button>
		</span>
		<span>
			<button
				id="edit"
				class="btn-icon variant-filled{state.kind === State.Display && state.editing
					? '-primary'
					: ''}"
				type="button"
				aria-expanded="false"
				title={$_('.action.edit')}
				disabled={state.kind === State.Create}
				on:click={() => {
					if (state.kind === State.Display) state.editing = true;
				}}
			>
				<i class="fa-solid fa-pen-to-square"></i>
			</button>
		</span>
	</div>

	<BookDisplay bind:this={bookDisplay} editable={state.kind === State.Create || state.editing} />

	<div class="p-2 pt-0 flex flex-wrap space-x-2 justify-center">
		{#if state.kind === State.Create || (state.kind === State.Display && state.editing)}
			<button
				id="book-abort-button"
				type="button"
				class="btn variant-filled mt-2"
				on:click={() => {
					if (state.kind === State.Display) {
						state.editing = false;
						bookDisplay.setBook(state.book);
					} else {
						onChangeInner(null);
					}
				}}
			>
				{$_('.action.cancel')}
			</button>
		{/if}

		{#if state.kind === State.Create}
			<button
				id="book-add-button"
				class="btn variant-filled-primary mt-2"
				type="button"
				on:click={() => (addResponse = add())}
			>
				<Spinner response={addResponse} />
				{$_('.action.add')}
			</button>
		{:else if state.kind === State.Display && state.editing}
			<button
				id="book-confirm-button"
				type="button"
				class="btn variant-filled-primary mt-2"
				on:click={() => (editResponse = edit())}
			>
				<Spinner response={editResponse} />
				{$_('.action.apply')}
			</button>
			<button
				class="btn variant-filled-error mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => (delResponse = del())}
			>
				<Spinner response={delResponse} />
				{$_('.action.delete')}
			</button>
		{:else if state.kind === State.Display && !state.editing}
			{#if state.book.reservation}
				{#if !state.book.borrower}
					<button
						class="btn variant-filled-primary mt-2"
						type="button"
						aria-expanded="false"
						on:click={() => {
							if (state.kind === State.Display) lendModalOpen(state.book.reservation);
						}}
					>
						{$_('.book.lend.to', { values: { '0': state.book.reservation } })}
					</button>
				{/if}
				<button
					class="btn variant-filled-error mt-2"
					type="button"
					aria-expanded="false"
					on:click={() => (releaseResponse = release())}
				>
					<Spinner response={releaseResponse} />
					{$_('.book.delete-reservation')}
				</button>
			{:else if state.book.borrower}
				<button
					class="btn variant-filled-primary mt-2"
					type="button"
					aria-expanded="false"
					on:click={reserveModalOpen}
				>
					{$_('.book.reserve')}
				</button>
				<button
					class="btn variant-filled-primary mt-2"
					type="button"
					aria-expanded="false"
					on:click={() => {
						if (state.kind === State.Display) lendModalOpen(state.book.borrower);
					}}
				>
					{$_('.book.renew')}
				</button>
			{:else if state.book.borrowable}
				<button
					class="btn variant-filled-primary mt-2"
					type="button"
					aria-expanded="false"
					on:click={() => lendModalOpen('')}
				>
					{$_('.book.lend')}
				</button>
			{/if}
			{#if state.book.borrower}
				<button
					class="btn variant-filled-error mt-2"
					type="button"
					aria-expanded="false"
					on:click={() => (returnResponse = returnBack())}
				>
					<Spinner response={returnResponse} />
					{$_('.book.revoke')}
				</button>
			{/if}
		{/if}
	</div>
</div>
