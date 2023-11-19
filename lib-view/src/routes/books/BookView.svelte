<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";

	import Spinner from "../../components/basic/Spinner.svelte";
	import LendDialog from "./LendDialog.svelte";
	import ReserveDialog from "./ReserveDialog.svelte";
	import MailDialog from "./MailDialog.svelte";
	import BookDisplay from "./BookDisplay.svelte";

	export let active: api.Book | null;
	export let editable: boolean = true;
	export var reload: (() => Promise<void>) | undefined;

	let lendDialog: LendDialog;
	let reserveDialog: ReserveDialog;
	let mailDialog: MailDialog;

	let bookDisplay: BookDisplay;

	$: if (active || !active) {
		if (bookDisplay) {
			bookDisplay.setBook(active);
			if (active) editable = false;
		}
	}
	$: if (active === null) editable = true;

	let addResponse: Promise<any>;
	async function add() {
		if (active === null) {
			let book = bookDisplay.getBook();
			await api.book_add(book);
			await onChange(book);
		}
	}

	let editResponse: Promise<any>;
	async function edit() {
		if (active !== null) {
			let book = bookDisplay.getBook();
			await api.book_update(active.id, book);
			await onChange(book);
		}
	}

	let deleteResponse: Promise<any>;
	async function del() {
		if (active !== null) {
			await api.book_delete(active.id);
			await onChange(null);
		}
	}

	let retResponse: Promise<any>;
	async function return_back() {
		if (active !== null) {
			let book = await api.return_back(active.id);
			await onChange(book);
			if (book.reservation) mailDialog.open(book);
		}
	}

	let releaseResponse: Promise<any>;
	async function release() {
		if (active !== null) {
			let book = await api.release(active.id);
			await onChange(book);
		}
	}

	async function onChange(book: api.Book | null) {
		active = book;
		bookDisplay.setBook(book);
		if (reload) await reload();
		editable = false;
	}
</script>

<BookDisplay bind:this={bookDisplay} {editable} />

{#if editable}
	{#if !active}
		<button
			id="book-add-button"
			class="btn btn-outline-primary mt-2"
			type="button"
			on:click={() => (addResponse = add())}
		>
			<Spinner response={addResponse} />
			{$_(".action.add")}
		</button>
	{:else}
		<button
			id="book-abort-button"
			type="button"
			class="btn btn-outline-secondary mt-2"
			hidden={!editable}
			on:click={() => {
				editable = active == null;
				bookDisplay.setBook(active);
			}}
		>
			{$_(".action.cancel")}
		</button>
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
	{/if}
{:else if active}
	<button
		id="edit"
		class="btn btn-outline-primary mt-2"
		class:active={editable}
		type="button"
		aria-expanded="false"
		title={$_(".action.edit")}
		disabled={!active}
		on:click={() => (editable = !editable)}
	>
		<i class="bi bi-pencil-square" />
	</button>

	{#if active.reservation}
		{#if !active.borrower}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => lendDialog.open(active?.reservation)}
			>
				{$_(".book.lend.to", { values: { "0": active.reservation } })}
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
	{:else if active.borrower}
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
			on:click={() => lendDialog.open(active?.borrower)}
		>
			{$_(".book.renew")}
		</button>
	{:else if active.borrowable}
		<button
			class="btn btn-outline-primary mt-2"
			type="button"
			aria-expanded="false"
			on:click={() => lendDialog.open()}
		>
			{$_(".book.lend")}
		</button>
	{/if}
	{#if active.borrower}
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

<LendDialog bind:this={lendDialog} bookId={active?.id ?? ""} {onChange} />
<ReserveDialog bind:this={reserveDialog} bookId={active?.id ?? ""} {onChange} />
<MailDialog bind:this={mailDialog} />
