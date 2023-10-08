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
		borrowed!: boolean;
		borrowable!: boolean;
	}
</script>

<script lang="ts">
	export let book: Book | null;
	export let isNew: boolean = false;

	let editable: boolean = false;

	let id: string = "";
	let isbn: string = "";
	let title: string = "";
	let publisher: string = "";
	let authors: string = "";
	let costs: number = 0;
	let year: number = 2023;
	let category: string = "None"; //temporary - todo: add categories
	let note: string | undefined = undefined;
	let borrowed: boolean = false;
	let borrowable: boolean = true;

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
				borrowed = book.borrowed;
				borrowable = book.borrowable;
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
			borrowed = false;
			borrowable = true;
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
		console.log("Delete:", book?.id);
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
			borrowed,
			borrowable
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
			title="Edit"
			on:click={() => {
				editable = true;
				isNew = false;
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentCol ps-0or"
				class="bi bi-pencil-square"
				viewBox="0 0 16 16"
			>
				<path
					d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z"
				/>
				<path
					fill-rule="evenodd"
					d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5v11z"
				/>
			</svg>
		</button>
		<button
			id="cancel"
			class="btn btn-outline-secondary"
			type="button"
			aria-expanded="false"
			title="Close"
			on:click={async () => {
				book = null;
				isNew = false;
				editable = false;
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentCol ps-0or"
				class="bi bi-x-lg"
				viewBox="0 0 16 16"
			>
				<path
					d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"
				/>
			</svg>
		</button>
	</div>

	<div class="row pt-1 m-0">
		<div class="col ps-0">
			<label for="id" class="form-label">Id</label>
			<div class="input-group" id="id">
				<input
					type="text"
					class="form-control"
					placeholder="Id"
					aria-label="Id"
					readonly={!editable}
					bind:value={id}
				/>
				<button
					type="button"
					class="btn btn-outline-secondary"
					title="Generate Id"
					disabled={!editable}
					on:click={() => console.log("Generate Id")}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-arrow-repeat"
						viewBox="0 0 16 16"
					>
						<path
							d="M11.534 7h3.932a.25.25 0 0 1 .192.41l-1.966 2.36a.25.25 0 0 1-.384 0l-1.966-2.36a.25.25 0 0 1 .192-.41zm-11 2h3.932a.25.25 0 0 0 .192-.41L2.692 6.23a.25.25 0 0 0-.384 0L.342 8.59A.25.25 0 0 0 .534 9z"
						/>
						<path
							fill-rule="evenodd"
							d="M8 3c-1.552 0-2.94.707-3.857 1.818a.5.5 0 1 1-.771-.636A6.002 6.002 0 0 1 13.917 7H12.9A5.002 5.002 0 0 0 8 3zM3.1 9a5.002 5.002 0 0 0 8.757 2.182.5.5 0 1 1 .771.636A6.002 6.002 0 0 1 2.083 9H3.1z"
						/>
					</svg>
				</button>
			</div>
		</div>
		<div class="col ps-0">
			<label for="isbn" class="form-label">ISBN</label>
			<div class="input-group" id="isbn">
				<input
					type="text"
					class="form-control"
					placeholder="ISBN"
					aria-label="ISBN"
					readonly={!editable}
					bind:value={isbn}
				/>
				<button
					type="button"
					class="btn btn-outline-secondary"
					title="Autofill"
					disabled={!editable}
					on:click={() => console.log("Autofill")}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-upload"
						viewBox="0 0 16 16"
					>
						<path
							d="M.5 9.9a.5.5 0 0 1 .5.5v2.5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2.5a.5.5 0 0 1 1 0v2.5a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-2.5a.5.5 0 0 1 .5-.5z"
						/>
						<path
							d="M7.646 1.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1-.708.708L8.5 2.707V11.5a.5.5 0 0 1-1 0V2.707L5.354 4.854a.5.5 0 1 1-.708-.708l3-3z"
						/>
					</svg>
				</button>
			</div>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="title" class="form-label">Title</label>
			<input
				id="title"
				type="text"
				class="form-control"
				placeholder="Title"
				aria-label="Title"
				readonly={!editable}
				bind:value={title}
			/>
		</div>
		<div class="col ps-0">
			<label for="publisher" class="form-label">Publisher</label>
			<input
				id="publisher"
				type="text"
				class="form-control"
				placeholder="Publisher"
				aria-label="Publisher"
				readonly={!editable}
				bind:value={publisher}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="authors" class="form-label">Authors</label>
			<input
				id="authors"
				type="text"
				class="form-control"
				placeholder="Authors"
				aria-label="Authors"
				readonly={!editable}
				bind:value={authors}
			/>
		</div>
		<div class="col ps-0">
			<label for="costs" class="form-label">Costs</label>
			<input
				id="costs"
				type="number"
				class="form-control"
				placeholder="Costs"
				aria-label="Costs"
				readonly={!editable}
				bind:value={costs}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="year" class="form-label">Year</label>
			<input
				id="year"
				type="number"
				class="form-control"
				placeholder="Year"
				aria-label="Year"
				readonly={!editable}
				bind:value={year}
			/>
		</div>
		<div class="col ps-0">
			<!--  Todo: General Selector  -->
			<label for="category" class="form-label">Category</label>
			<input
				id="category"
				type="text"
				class="form-control"
				placeholder="Category"
				aria-label="Category"
				readonly={!editable}
				bind:value={category}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="note" class="form-label">Note</label>
			<input
				id="note"
				type="text"
				class="form-control"
				placeholder="Note"
				aria-label="Note"
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
			<label class="form-check-label" for="borrowable">Borrowable</label>
		</div>
	</div>
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
		Cancel
	</button>
	<button
		id="book-add-button"
		class="btn btn-outline-primary mt-2"
		type="button"
		hidden={!(editable && isNew)}
		on:click={() => (addResponse = add())}
	>
		{#await addResponse}
			<span
				id="book-add-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Add
	</button>
	<button
		id="book-confirm-button"
		type="button"
		class="btn btn-outline-primary mt-2"
		hidden={!(editable && !isNew)}
		on:click={() => (editResponse = edit())}
	>
		{#await editResponse}
			<span
				id="book-confirm-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		Confirm
	</button>
	<button
		id="del"
		class="btn btn-outline-danger mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(editable && !isNew)}
		on:click={async () => {
			await del();
		}}>Delete</button
	>
	<button
		id="del"
		class="btn btn-outline-primary mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew && !(book ? book.borrowed : false))}
		on:click={async () => {
			console.log("Initiate Borrow");
		}}>Borrow</button
	>
	<button
		id="del"
		class="btn btn-outline-primary mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew && book ? book.borrowed : false)}
		on:click={async () => {
			console.log("Initiate Reserve");
		}}>Reserve</button
	>
	<button
		id="del"
		class="btn btn-outline-primary mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew && book ? book.borrowed : false)}
		on:click={async () => {
			console.log("Initiate Extend");
		}}>Extend</button
	>
	<button
		id="del"
		class="btn btn-outline-danger mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew && book ? book.borrowed : false)}
		on:click={async () => {
			console.log("Initiate Give Back");
		}}>Give Back</button
	>
{/if}
