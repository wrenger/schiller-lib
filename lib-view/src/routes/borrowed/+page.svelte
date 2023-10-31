<script lang="ts">
	import { _ } from "svelte-i18n";
	import type { Book } from "../books/BookView.svelte";
	import { onMount } from "svelte";
	import Request from "../../components/basic/Request.svelte";
	import type { User } from "../users/UserView.svelte";
	import { DateTime } from "luxon";
	import BorrowedList from "./BorrowedList.svelte";

	let borrowedBooks: Promise<[Book, User][]>;
	let overdoneBooks: Promise<[Book, User][]>;
	let r: Request;

	onMount(() => {
		borrowedBooks = r.request("api/borrowed", "GET", null);
		overdoneBooks = r.request("api/overdues", "GET", null);
	});
</script>

<svelte:head>
	<title>{$_(".book.borrowed")}</title>
	<meta name="description" content={$_(".book.borrowed")} />
</svelte:head>

<Request bind:this={r} />

<div class="card h-100">
	<div class="card-header d-flex justify-content-between">
		<ul class="nav nav-tabs card-header-tabs" id="borrowedBooksTab" role="tablist">
			<li class="nav-item">
				<a
					class="nav-link active"
					id="all-tab"
					data-bs-toggle="tab"
					data-bs-target="#all-borrowed"
					href="#all-borrowed"
					role="tab"
					aria-controls="all-borrowed"
					aria-selected="true">{$_(".book.all.borrowed")}</a
				>
			</li>
			<li class="nav-item">
				<a
					class="nav-link"
					id="over-tab"
					data-bs-toggle="tab"
					data-bs-target="#all-overdues"
					href="#over-tab"
					role="tab"
					aria-controls="all-overdues"
					aria-selected="true">{$_(".book.all.overdues")}</a
				>
			</li>
		</ul>
		<div class="d-flex align-items-center">
			<span>{$_(".book.period.date")} / {$_(".book.period.days")}</span>
		</div>
	</div>
	<div class="tab-content full" id="borrowedBooksTabContent">
		<div
			class="tab-pane fade show active"
			id="all-borrowed"
			role="tabpanel"
			aria-labelledby="all-tab"
		>
			<BorrowedList promise={borrowedBooks} let:item={[book, user]}>
				<a
					class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
					href="books?i={book.id}"
				>
					<div class="d-flex flex-column">
						<p class="mb-1 text-truncate h"><strong>{book.title}</strong></p>
						<p class="mb-0 text-truncate h">
							{$_(".book.borrowed.by.short", {
								values: { "0": `${user.forename} ${user.surname}` }
							})}
						</p>
					</div>
					<div class="d-flex flex-column">
						<p class="mb-0 text-truncate h">
							{$_(".book.period", {
								values: {
									"0": DateTime.fromISO(book.deadline ? book.deadline : "").toLocaleString(),
									"1": parseInt(
										DateTime.fromISO(book.deadline ? book.deadline : "")
											.diff(DateTime.now(), "days")
											.days.toLocaleString()
									)
								}
							})}
						</p>
					</div>
				</a>
			</BorrowedList>
		</div>
		<div class="tab-pane fade" id="all-overdues" role="tabpanel" aria-labelledby="over-tab">
			<BorrowedList promise={overdoneBooks} let:item={[book, user]}>
				<a
					class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
					href={`books?i=${book.id}`}
				>
					<div class="d-flex flex-column">
						<p class="mb-1 text-truncate h"><strong>{book.title}</strong></p>
						<p class="mb-0 text-truncate h">
							{$_(".book.overdone.by.short", {
								values: { "0": `${user.forename} ${user.surname}` }
							})}
						</p>
					</div>
					<div class="d-flex flex-column">
						<p class="mb-0 text-truncate h">
							{$_(".book.period", {
								values: {
									"0": DateTime.fromISO(book.deadline ? book.deadline : "").toLocaleString(),
									"1": parseInt(
										(-DateTime.fromISO(book.deadline ? book.deadline : "").diff(
											DateTime.now(),
											"days"
										).days).toLocaleString()
									)
								}
							})}
						</p>
					</div>
				</a>
			</BorrowedList>
		</div>
	</div>
</div>

<style>
	.full {
		overflow: scroll;
		height: calc(100% - 52px);
	}
</style>
