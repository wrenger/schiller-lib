<script lang="ts">
	import { _ } from "svelte-i18n";
	import Spinner from "../../components/basic/Spinner.svelte";
	import Dialog from "../../components/basic/Dialog.svelte";
	import UserSelect from "../users/UserSelect.svelte";
	import DateField from "../../components/basic/DateField.svelte";
	import { DateTime } from "luxon";
	import { settingsGlobal } from "$lib/store";
	import CategorySelect from "../../components/basic/CategorySelect.svelte";
	import api from "$lib/api";

	export let book: api.Book | null;
	export let isNew: boolean = false;
	export var reload: (() => Promise<void>) | undefined;

	let editable: boolean = false;

	let lendDialog: Dialog;
	let reserveDialog: Dialog;
	let mailDialog: Dialog;

	let id: string = "";
	let isbn: string = "";
	let title: string = "";
	let publisher: string = "";
	let authors: string = "";
	let costs: number = 0;
	let year: number = 2023;
	let category: string = "None";
	let note: string = "";
	let borrowable: boolean = true;
	let borrower: string | undefined = undefined;
	let deadline: DateTime | undefined = undefined;
	let reservation: string | undefined = undefined;

	let period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
	settingsGlobal.subscribe((s) => (period = DateTime.local().plus({ days: s.borrowing_duration })));

	let gonnaBorrow: string | undefined;
	let gonnaReserve: string | undefined;

	$: if (editable || isNew || !editable || !isNew) setBook(book);
	$: if (isNew) editable = true;

	function setBook(book: api.Book | null) {
		if (!isNew) {
			if (book) {
				id = book.id;
				isbn = book.isbn;
				title = book.title;
				publisher = book.publisher;
				authors = book.authors.join(", ");
				costs = book.costs;
				year = book.year;
				category = book.category;
				note = book.note;
				borrowable = book.borrowable;
				borrower = book.borrower;
				deadline = DateTime.fromISO(book.deadline || "");
				reservation = book.reservation;
			}
		} else {
			id = "";
			isbn = "";
			title = "";
			publisher = "";
			authors = "";
			costs = 0;
			year = DateTime.now().year;
			category = "None";
			note = "";
			borrowable = true;
			borrower = undefined;
			deadline = undefined;
			reservation = undefined;
		}
	}

	function getBook(): api.Book {
		return {
			id,
			isbn,
			title,
			publisher,
			authors: authors.split(",").map((a) => a.trim()),
			costs,
			year,
			category,
			note,
			borrowable,
			borrower: borrower ?? "",
			deadline: deadline ? deadline?.toISODate() || "" : "",
			reservation: reservation ?? ""
		};
	}

	let addResponse: Promise<any>;
	async function add() {
		await api.book_add(getBook());
		await onChange();
	}

	let editResponse: Promise<any>;
	async function edit() {
		await api.book_update(id, getBook());
		await onChange();
	}

	let deleteResponse: Promise<any>;
	async function del() {
		await api.book_delete(book?.id || "");
		await onChange();
	}

	let lendResponse: Promise<any>;
	async function lend() {
		await api.lend(id, gonnaBorrow ? gonnaBorrow : "", period ? period?.toISODate() || "" : "");
		period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
		reservation = "";
		lendDialog.close();
		await onChange();
	}

	let retResponse: Promise<any>;
	async function return_back() {
		await api.return_back(id);
		borrower = undefined;
		deadline = undefined;
		await onChange();
		if (reservation) mailDialog.open();
	}

	let reserveResponse: Promise<any>;
	async function reserve() {
		await api.reserve(id, gonnaReserve || "");
		await onChange();
		reserveDialog.close();
	}

	let releaseResponse: Promise<any>;
	async function release() {
		await api.release(id);
		await onChange();
	}

	let mailResponse: Promise<any>;
	async function mail() {
		let user = await api.user_fetch(reservation || "");

		await api.mail([
			{
				account: reservation || "",
				subject: $settingsGlobal.mail_info_subject
					.replace(/\{booktitle\}/g, title)
					.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
				body: $settingsGlobal.mail_info_content
					.replace(/\{booktitle\}/g, title)
					.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
			}
		]);

		await onChange();
		mailDialog.close();
	}

	async function onChange() {
		book = getBook();
		if (reload) await reload();
		editable = false;
		isNew = false;
	}
</script>

{#if book || isNew}
	<div class="card-header d-flex justify-content-between">
		<button
			id="edit"
			class="btn btn-outline-primary"
			class:active={editable && !isNew}
			type="button"
			aria-expanded="false"
			title={$_(".action.edit")}
			disabled={!book}
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
			on:click={() => {
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
					on:click={async () => {
						id = await api.book_id(getBook());
					}}
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
					on:click={async () => {
						let data = await api.book_fetch(isbn);
						title = data.title || "";
						publisher = data.publisher || "";
						authors = data.authors?.join(",") || "";
						costs = data.costs || 0;
					}}
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
			<CategorySelect bind:value={category} disabled={!editable} label={"Category"} />
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0 pe-0">
			<label for="note" class="form-label">{$_(".book.note")}</label>
			<textarea
				id="note"
				class="form-control"
				aria-label={$_(".book.note")}
				readonly={!editable}
				bind:value={note}
				rows="3"
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
					values: {
						"0": borrower,
						"1": deadline.toLocaleString()
					}
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

	{#if editable}
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
		{#if isNew}
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
	{:else if !isNew}
		{#if reservation}
			{#if !borrower}
				<button
					class="btn btn-outline-primary mt-2"
					type="button"
					aria-expanded="false"
					on:click={() => {
						lendDialog.open();
						gonnaBorrow = reservation;
					}}
				>
					{$_(".book.lend.to", { values: { "0": reservation } })}
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
		{:else if borrower}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
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
				on:click={() => {
					gonnaBorrow = borrower;
					lendDialog.open();
				}}
			>
				{$_(".book.renew")}
			</button>
		{:else if borrowable}
			<button
				class="btn btn-outline-primary mt-2"
				type="button"
				aria-expanded="false"
				on:click={() => {
					gonnaBorrow = "";
					lendDialog.open();
				}}
			>
				{$_(".book.lend")}
			</button>
		{/if}
		{#if borrower}
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
{/if}

<Dialog
	bind:this={lendDialog}
	onCancel={() => (period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration }))}
>
	<span slot="header"><h5 class="mb-0">{$_(".book.lend")}</h5></span>
	<span slot="body">
		<UserSelect label={$_(".user")} placeholder={$_(".user.account")} bind:value={gonnaBorrow} />
		<label for="period" class="sform-label">{$_(".book.lend.period")}</label>
		<DateField bind:date={period} id="period" />
	</span>
	<span slot="footer">
		<button
			id="book-confirm-button"
			type="button"
			class="btn btn-primary"
			on:click={() => (lendResponse = lend())}
		>
			<Spinner response={lendResponse} />
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
		<button type="button" class="btn btn-primary" on:click={() => (reserveResponse = reserve())}>
			<Spinner response={reserveResponse} />
			{$_(".action.apply")}
		</button>
	</span>
</Dialog>

<Dialog bind:this={mailDialog}>
	<span slot="header"><h5 class="mb-0">{$_(".alert.confirm")}</h5></span>
	<span slot="body">{$_(".book.revoke.reminder", { values: { "0": reservation } })}</span>
	<span slot="footer">
		<button type="button" class="btn btn-primary" on:click={() => (mailResponse = mail())}>
			<Spinner response={mailResponse} />
			{$_(".action.ok")}
		</button>
	</span>
</Dialog>
