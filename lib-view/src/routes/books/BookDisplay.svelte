<script lang="ts">
	import { _ } from "svelte-i18n";
	import api from "$lib/api";

	import { DateTime } from "luxon";
	import CategorySelect from "../../components/basic/CategorySelect.svelte";

	export let editable: boolean = false;

	let id = "";
	let isbn = "";
	let title = "";
	let publisher = "";
	let authors = "";
	let costs = 0;
	let year = DateTime.now().year;
	let category = "None";
	let note = "";
	let borrowable = true;
	let borrower = "";
	let deadline: DateTime | null = null;
	let reservation = "";

	function defaultBook(): api.Book {
		return {
			id: "",
			isbn: "",
			title: "",
			publisher: "",
			authors: [],
			costs: 0,
			year: DateTime.now().year,
			category: "None",
			note: "",
			borrowable: true,
			borrower: "",
			deadline: "",
			reservation: ""
		};
	}

	export function setBook(b: api.Book | null) {
		b ??= defaultBook();

		id = b.id;
		isbn = b.isbn;
		title = b.title;
		publisher = b.publisher;
		authors = b.authors.join(", ");
		costs = b.costs;
		year = b.year;
		category = b.category;
		note = b.note;
		borrowable = b.borrowable;
		borrower = b.borrower;
		deadline = DateTime.fromISO(b.deadline || "");
		reservation = b.reservation;
	}

	export function getBook(): api.Book {
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
</script>

<div class="row m-1">
	<div class="col ps-0 pe-0">
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
</div>
<div class="row pt-1 m-1">
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
<div class="row m-1">
	<div class="col ps-0 pe-0">
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
</div>
<div class="row m-1">
	<div class="col ps-0">
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
<div class="row m-1">
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
<div class="row m-1">
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
<div class="row m-1 pt-1">
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

{#if !editable}
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
