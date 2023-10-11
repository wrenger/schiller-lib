<script lang="ts" context="module">
	export class Book {
		id!: string;
		isbn!: string;
		title!: string;
		publisher!: string;
		authors!: string[];
		costs!: number;
		year!: number;
		category!: string; //temporary - todo: add categories
		note?: string;
		borrowable!: boolean;
		borrower?: string;
		deadline?: Date;
		reservation?: string;
	}
</script>

<script lang="ts">
	import { _ } from "svelte-i18n";
	import Spinner from "../../components/basic/Spinner.svelte";
	import Dialog from "../../components/basic/Dialog.svelte";
	import UserSelect from "../users/UserSelect.svelte";

	export let book: Book | null;
	export let isNew: boolean = false;

	let editable: boolean = false;

	let lendDialog: Dialog;
	let reserveDialog: Dialog;
	let confirmDialog: Dialog;

	let id: string = "";
	let isbn: string = "";
	let title: string = "";
	let publisher: string = "";
	let authors: string = "";
	let costs: number = 0;
	let year: number = 2023;
	let category: string = "None"; //temporary - todo: add categories
	let note: string | undefined = undefined;
	let borrowable: boolean = true;
	let borrower: string | undefined = "";
	let deadline: Date | undefined = undefined;
	let reservation: string | undefined = "";

	let period: number = 28;
	let gonnaBorrow: string | undefined;
	let gonnaReserve: string | undefined;

	$: if (editable || isNew || !editable || !isNew) setBook(book);
	$: if (isNew) editable = true;

	function setBook(book: Book | null) {
		if (!isNew) {
			if (book) {
				id = book.id;
				isbn = book.isbn;
				title = book.title;
				publisher = book.publisher;
				authors = book.authors.join(",");
				costs = book.costs;
				year = book.year;
				category = book.category;
				note = book.note;
				borrowable = book.borrowable;
				borrower = book.borrower;
				deadline = book.deadline;
				reservation = book.reservation;
			}
		} else {
			id = "";
			isbn = "";
			title = "";
			publisher = "";
			authors = "";
			costs = 0;
			year = 2023;
			category = "None"; //temporary - todo: add categories
			note = undefined;
			borrowable = true;
			borrower = "";
			deadline = undefined;
			reservation = "";
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		onChange();
		console.log("Add:", book);
	}

	let editResponse: Promise<any>;
	async function edit() {
		onChange();
		console.log("Edit:", book);
	}
	export async function del() {
		console.log("Delete:", id);
		book = null;
		editable = false;
		isNew = false;
	}

	function onChange() {
		book = {
			id,
			isbn,
			title,
			publisher,
			authors: authors.split(","),
			costs,
			year,
			category,
			note,
			borrowable,
			borrower,
			deadline,
			reservation
		};
		editable = false;
		isNew = false;
	}
</script>

{#if book || isNew}
	<div class="card-header d-flex justify-content-between">
		<button
			id="edit"
			class="btn btn-outline-primary {editable && !isNew ? 'active' : ''}"
			type="button"
			aria-expanded="false"
			title={$_(".action.edit")}
			on:click={() => {
				editable = true;
				isNew = false;
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
			on:click={async () => {
				book = null;
				isNew = false;
				editable = false;
			}}
		>
			<i class="bi bi-x-lg" />
		</button>
	</div>

	<div class="row pt-1 m-0">
		<div class="col ps-0">
			<label for="id" class="form-label">{$_(".book.id")}</label>
			<div class="input-group" id="id">
				<input
					type="text"
					class="form-control"
					placeholder={$_(".book.id")}
					aria-label={$_(".book.id")}
					readonly={!editable}
					bind:value={id}
				/>
				<button
					type="button"
					class="btn btn-outline-secondary"
					title={$_(".book.id.action")}
					disabled={!editable}
					on:click={() => console.log("Generate Id")}
				>
					<i class="bi bi-arrow-repeat" />
				</button>
			</div>
		</div>
		<div class="col ps-0 pe-0">
			<label for="isbn" class="form-label">{$_(".book.isbn")}</label>
			<div class="input-group" id="isbn">
				<input
					type="text"
					class="form-control"
					placeholder={$_(".book.isbn")}
					aria-label={$_(".book.isbn")}
					readonly={!editable}
					bind:value={isbn}
				/>
				<button
					type="button"
					class="btn btn-outline-secondary"
					title={$_(".book.request")}
					disabled={!editable}
					on:click={() => console.log("Autofill")}
				>
					<i class="bi bi-upload" />
				</button>
			</div>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="title" class="form-label">{$_(".book.title")}</label>
			<input
				id="title"
				type="text"
				class="form-control"
				placeholder={$_(".book.title")}
				aria-label={$_(".book.title")}
				readonly={!editable}
				bind:value={title}
			/>
		</div>
		<div class="col ps-0 pe-0">
			<label for="publisher" class="form-label">{$_(".book.publisher")}</label>
			<input
				id="publisher"
				type="text"
				class="form-control"
				placeholder={$_(".book.publisher")}
				aria-label={$_(".book.publisher")}
				readonly={!editable}
				bind:value={publisher}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="authors" class="form-label">{$_(".book.authors")}</label>
			<input
				id="authors"
				type="text"
				class="form-control"
				placeholder={$_(".book.authors")}
				aria-label={$_(".book.authors")}
				readonly={!editable}
				bind:value={authors}
			/>
		</div>
		<div class="col ps-0 pe-0">
			<label for="costs" class="form-label">{$_(".book.costs")}</label>
			<input
				id="costs"
				type="number"
				step="0.1"
				class="form-control"
				placeholder={$_(".book.costs")}
				aria-label={$_(".book.costs")}
				readonly={!editable}
				bind:value={costs}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="year" class="form-label">{$_(".book.year")}</label>
			<input
				id="year"
				type="number"
				class="form-control"
				placeholder={$_(".book.year")}
				aria-label={$_(".book.year")}
				readonly={!editable}
				bind:value={year}
			/>
		</div>
		<div class="col ps-0 pe-0">
			<!--  Todo: General Selector  -->
			<label for="category" class="form-label">{$_(".category")}</label>
			<input
				id="category"
				type="text"
				class="form-control"
				placeholder={$_(".category")}
				aria-label={$_(".category")}
				readonly={!editable}
				bind:value={category}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0 pe-0">
			<label for="note" class="form-label">{$_(".book.note")}</label>
			<input
				id="note"
				type="text"
				class="form-control"
				placeholder={$_(".book.note")}
				aria-label={$_(".book.note")}
				readonly={!editable}
				bind:value={note}
			/>
		</div>
	</div>
	<div class="row m-0 pt-1">
		<div class="form-check">
			<input
				class="form-check-input"
				type="checkbox"
				value=""
				id="borrowable"
				bind:checked={borrowable}
				disabled={!editable}
			/>
			<label class="form-check-label" for="borrowable">{$_(".book.borrowable")}</label>
		</div>
	</div>

	{#if !editable && !isNew}
		{#if borrower && deadline}
			<div class="alert alert-light mb-0" role="alert">
				{$_(".book.borrowed.by", {
					values: { "0": borrower, "1": deadline.toLocaleDateString("en-GB").replace(/\//g, ".") }
				})}
			</div>
			{#if reservation}
				<div class="alert alert-light mb-0 mt-1" role="alert">
					{$_(".book.reserved.by", { values: { "0": reservation } })}
				</div>
			{/if}
		{:else if reservation}
			<div class="alert alert-light mb-0" role="alert">
				{$_(".book.reserved.by", { values: { "0": reservation } })}
			</div>
		{:else if borrowable}
			<div class="alert alert-light mb-0" role="alert">
				{$_(".book.available.long")}
			</div>
		{:else}
			<div class="alert alert-light mb-0" role="alert">
				{$_(".book.not-borrowable")}
			</div>
		{/if}
	{/if}

	<button
		id="book-abort-button"
		type="button"
		class="btn btn-outline-secondary mt-2"
		hidden={!editable}
		on:click={() => {
			setBook(book);
			editable = false;
			isNew = false;
		}}
	>
		{$_(".action.cancel")}
	</button>
	<button
		id="book-add-button"
		class="btn btn-outline-primary mt-2"
		type="button"
		hidden={!(editable && isNew)}
		on:click={() => (addResponse = add())}
	>
		<Spinner response={addResponse} />
		{$_(".action.add")}
	</button>
	<button
		id="book-confirm-button"
		type="button"
		class="btn btn-outline-primary mt-2"
		hidden={!(editable && !isNew)}
		on:click={() => (editResponse = edit())}
	>
		<Spinner response={editResponse} />
		{$_(".action.apply")}
	</button>
	<button
		class="btn btn-outline-danger mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(editable && !isNew)}
		on:click={async () => {
			await del();
		}}
	>
		{$_(".action.delete")}
	</button>

	{#if !editable && !isNew}
		{#if reservation}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				hidden={!!(borrower ?? false)}
				on:click={() => {
					lendDialog.open();
					gonnaBorrow = reservation;
				}}
			>
				{$_(".book.lend.to", { values: { "0": reservation } })}
			</button>
		{:else}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				hidden={!(!(borrower ?? false) && borrowable)}
				on:click={() => {
					gonnaBorrow = "";
					lendDialog.open();
				}}
			>
				{$_(".book.lend")}
			</button>
		{/if}
		<button
			class="btn btn-outline-danger mt-2"
			type="button"
			aria-expanded="false"
			hidden={!reservation}
			on:click={() => {
				reservation = undefined;
				editResponse = edit();
			}}
		>
			<Spinner response={editResponse} />
			{$_(".book.delete-reservation")}
		</button>
		{#if book && book.borrower}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				hidden={!!reservation}
				on:click={() => {
					gonnaReserve = "";
					reserveDialog.open();
				}}
			>
				{$_(".book.reserve")}
			</button>
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				hidden={!!reservation}
				on:click={() => {
					gonnaBorrow = borrower;
					lendDialog.open();
				}}
			>
				{$_(".book.renew")}
			</button>
			<button
				class="btn btn-outline-danger mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => {
					borrower = undefined;
					deadline = new Date();
					editResponse = edit();
					if (reservation) confirmDialog.open();
				}}
			>
				<Spinner response={editResponse} />
				{$_(".book.revoke")}
			</button>
		{/if}
	{/if}
{/if}

<Dialog bind:this={lendDialog}>
	<span slot="header"><h5 class="mb-0">{$_(".book.lend")}</h5></span>
	<span slot="body">
		<UserSelect label={$_(".user")} placeholder={$_(".user.account")} bind:value={gonnaBorrow} />
		<label for="period" class="form-label">{$_(".book.lend.period")}</label>
		<input
			id="period"
			type="number"
			class="form-control"
			placeholder={$_(".book.lend.period")}
			aria-label={$_(".book.lend.period")}
			bind:value={period}
		/>
	</span>
	<span slot="footer">
		<button
			id="book-confirm-button"
			type="button"
			class="btn btn-primary"
			on:click={() => {
				deadline = new Date(new Date().getTime() + period * 24 * 60 * 60 * 1000);
				borrower = gonnaBorrow;
				reservation = "";
				editResponse = edit();
				lendDialog.close();
			}}
		>
			<Spinner response={editResponse} />
			{$_(".action.apply")}
		</button>
	</span>
</Dialog>

<Dialog bind:this={reserveDialog}>
	<span slot="header"><h5 class="mb-0">{$_(".book.reserve")}</h5></span>
	<span slot="body">
		<UserSelect label={$_(".user")} placeholder={$_(".user.account")} bind:value={gonnaReserve} />
	</span>
	<span slot="footer">
		<button
			type="button"
			class="btn btn-primary"
			on:click={() => {
				reservation = gonnaReserve;
				editResponse = edit();
				reserveDialog.close();
			}}
		>
			<Spinner response={editResponse} />
			{$_(".action.apply")}
		</button>
	</span>
</Dialog>

<Dialog bind:this={confirmDialog}>
	<span slot="header"><h5 class="mb-0">{$_(".alert.confirm")}</h5></span>
	<span slot="body">{$_(".book.revoke.reminder", { values: { "0": reservation } })}</span>
	<span slot="footer">
		<button
			type="button"
			class="btn btn-primary"
			on:click={() => {
				console.log("Send Mail!");
				confirmDialog.close();
			}}
		>
			{$_(".action.ok")}
		</button>
	</span>
</Dialog>
