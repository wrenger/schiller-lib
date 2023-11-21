<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";

	import Spinner from "../../components/basic/Spinner.svelte";
	import LendDialog from "./LendDialog.svelte";
	import ReserveDialog from "./ReserveDialog.svelte";
	import MailDialog from "./MailDialog.svelte";
	import BookDisplay from "./BookDisplay.svelte";

	// look what they need to mimic a fraction of our power
	enum State {
		Display,
		Adding
	}
	interface Display {
		kind: State.Display;
		book: api.Book;
		editing: boolean;
	}
	interface Adding {
		kind: State.Adding;
	}
	let state: Display | Adding = { kind: State.Adding };

	export var onChange: ((b: api.Book | null) => void) | undefined;

	export function display(b: api.Book) {
		state = {
			kind: State.Display,
			book: b,
			editing: false
		};
	}
	export function adding() {
		state = { kind: State.Adding };
	}

	let lendDialog: LendDialog;
	let reserveDialog: ReserveDialog;
	let mailDialog: MailDialog;
	let bookDisplay: BookDisplay;

	$: if (state.kind == State.Display) {
		if (bookDisplay) {
			bookDisplay.setBook(state.book);
		}
	}
	$: if (state.kind == State.Adding) {
		if (bookDisplay) {
			bookDisplay.setBook(null);
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		if (state.kind === State.Adding) {
			let book = bookDisplay.getBook();
			await api.book_add(book);
			onChangeInner(book);
		}
	}

	let editResponse: Promise<any>;
	async function edit() {
		if (state.kind == State.Display) {
			let book = bookDisplay.getBook();
			await api.book_update(state.book.id, book);
			onChangeInner(book);
		}
	}

	let deleteResponse: Promise<any>;
	async function del() {
		if (state.kind == State.Display) {
			await api.book_delete(state.book.id);
			onChangeInner(null);
		}
	}

	let retResponse: Promise<any>;
	async function return_back() {
		if (state.kind == State.Display) {
			let book = await api.return_back(state.book.id);
			onChangeInner(book);
			if (book.reservation) mailDialog.open(book);
		}
	}

	let releaseResponse: Promise<any>;
	async function release() {
		if (state.kind == State.Display) {
			let book = await api.release(state.book.id);
			onChangeInner(book);
		}
	}

	function onChangeInner(book: api.Book | null) {
		if (book != null) {
			if (state.kind === State.Display) {
				state.book = book;
				state.editing = false;
				bookDisplay.setBook(book);
			}
			if (onChange) onChange(book);
		}
	}
</script>

<div class="card-header d-flex justify-content-between">
	<button
		id="edit"
		class="btn btn-outline-primary"
		class:active={state.kind === State.Display && state.editing}
		type="button"
		aria-expanded="false"
		title={$_(".action.edit")}
		disabled={state.kind === State.Adding}
		on:click={() => {
			if (state.kind === State.Display) state.editing = true;
		}}
	>
		<i class="bi bi-pencil-square" />
	</button>
	<button
		id="cancel"
		class="btn btn-outline-secondary"
		type="button"
		aria-expanded="false"
		title={$_(".action.close")}
		on:click={() => {
			if (onChange) onChange(null);
		}}
	>
		<i class="bi bi-x-lg" />
	</button>
</div>

<BookDisplay bind:this={bookDisplay} editable={state.kind === State.Adding || state.editing} />

<div class="card-footer text-center">
	{#if state.kind === State.Adding || (state.kind === State.Display && state.editing)}
		<button
			id="book-abort-button"
			type="button"
			class="btn btn-outline-secondary mt-2"
			on:click={() => {
				if (state.kind === State.Display) {
					state.editing = false;
					bookDisplay.setBook(state.book);
				} else {
					if (onChange) onChange(null);
				}
			}}
		>
			{$_(".action.cancel")}
		</button>
	{/if}

	{#if state.kind === State.Adding}
		<button
			id="book-add-button"
			class="btn btn-outline-primary mt-2"
			type="button"
			on:click={() => (addResponse = add())}
		>
			<Spinner response={addResponse} />
			{$_(".action.add")}
		</button>
	{:else if state.kind === State.Display && state.editing}
		<button
			id="book-confirm-button"
			type="button"
			class="btn btn-outline-primary mt-2"
			on:click={() => (editResponse = edit())}
		>
			<Spinner response={editResponse} />
			{$_(".action.apply")}
		</button>
		<button
			class="btn btn-outline-danger mt-2"
			type="button"
			aria-expanded="false"
			on:click={() => (deleteResponse = del())}
		>
			<Spinner response={deleteResponse} />
			{$_(".action.delete")}
		</button>
	{:else if state.kind === State.Display && !state.editing}
		{#if state.book.reservation}
			{#if !state.book.borrower}
				<button
					class="btn btn-outline-primary mt-2"
					type="button"
					aria-expanded="false"
					on:click={() => {
						if (state.kind === State.Display) lendDialog.open(state.book.reservation);
					}}
				>
					{$_(".book.lend.to", { values: { "0": state.book.reservation } })}
				</button>
			{/if}
			<button
				class="btn btn-outline-danger mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => (releaseResponse = release())}
			>
				<Spinner response={releaseResponse} />
				{$_(".book.delete-reservation")}
			</button>
		{:else if state.book.borrower}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => reserveDialog.open()}
			>
				{$_(".book.reserve")}
			</button>
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => {
					if (state.kind === State.Display) lendDialog.open(state.book.borrower);
				}}
			>
				{$_(".book.renew")}
			</button>
		{:else if state.book.borrowable}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => lendDialog.open()}
			>
				{$_(".book.lend")}
			</button>
		{/if}
		{#if state.book.borrower}
			<button
				class="btn btn-outline-danger mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => (retResponse = return_back())}
			>
				<Spinner response={retResponse} />
				{$_(".book.revoke")}
			</button>
		{/if}
	{/if}
</div>

<LendDialog
	bind:this={lendDialog}
	bookId={state.kind === State.Display ? state.book.id : ""}
	onChange={onChangeInner}
/>
<ReserveDialog
	bind:this={reserveDialog}
	bookId={state.kind === State.Display ? state.book.id : ""}
	onChange={onChangeInner}
/>
<MailDialog bind:this={mailDialog} />
