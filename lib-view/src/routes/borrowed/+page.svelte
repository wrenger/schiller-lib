<script lang="ts">
	import { _ } from "svelte-i18n";
	let borrowedBooks = [
		{ id: "BOOK XXXX 1", title: "Book 1", borrower: "User A" },
		{ id: "BOOK XXXX 2", title: "Book 2", borrower: "User B" }
	];
</script>

<svelte:head>
	<title>{$_(".book.borrowed")}</title>
	<meta name="description" content={$_(".book.borrowed")} />
</svelte:head>

<div class="container-full">
	<ul class="nav nav-tabs" id="borrowedBooksTab" role="tablist">
		<li class="nav-item">
			<a
				class="nav-link active"
				id="all-tab"
				data-bs-toggle="tab"
				data-bs-target="#all-borrowed"
				href="#all-borrowed"
				role="tab"
				aria-controls="all-borrowed"
				aria-selected="true">All Borrowed</a
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
				aria-selected="true">All Overdues</a
			>
		</li>
	</ul>

	<div class="tab-content" id="borrowedBooksTabContent">
		<div
			class="tab-pane fade show active"
			id="all-borrowed"
			role="tabpanel"
			aria-labelledby="all-tab"
		>
			<ul class="list-group mt-3">
				{#each borrowedBooks as book (book.id)}
					<a class="list-group-item list-group-item-action" href={`books?i=${book.id}`}>
						<strong>{book.title}</strong> - Borrowed by {book.borrower} until ? (date)
					</a>
				{/each}
			</ul>
		</div>
		<div class="tab-pane fade" id="all-overdues" role="tabpanel" aria-labelledby="over-tab">
			<ul class="list-group mt-3">
				{#each borrowedBooks as book (book.id)}
					<a class="list-group-item list-group-item-action" href={`books?i=${book.id}`}>
						<strong>{book.title}</strong> - Overdone by {book.borrower} with ? days
					</a>
				{/each}
			</ul>
		</div>
	</div>
</div>
