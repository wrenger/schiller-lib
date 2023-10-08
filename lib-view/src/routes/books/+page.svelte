<script lang="ts">
	import Container from "../../components/basic/Container.svelte";
	import BookList from "./BookList.svelte";
	import BookSearch, { BookParams } from "./BookSearch.svelte";
	import BookView, { Book } from "./BookView.svelte";

	let params: BookParams;
	let active: Book | null;
	let isNew: boolean;
</script>

<svelte:head>
	<title>Books</title>
	<meta name="description" content="Shows the Books" />
</svelte:head>

<Container>
	<span slot="list">
		<BookSearch bind:params />
		<BookList
			bind:active
			bind:isNew
			items={[
				{
					id: "ABCD EFGH 1",
					isbn: 1234567890,
					title: "Sample Book 1",
					publisher: "Publisher A",
					authors: ["John Doe", "Jane Smith"],
					costs: 10,
					year: 2021,
					category: "Fiction",
					borrowed: false,
					borrowable: true
				},
				{
					id: "WXYZ 1234 2",
					isbn: 9876543210,
					title: "Another Book",
					publisher: "Publisher B",
					authors: ["Alice Johnson"],
					costs: 15,
					year: 2020,
					category: "Non-Fiction",
					borrowed: false,
					borrowable: true
				},
				{
					id: "QWER TYUI 3",
					isbn: 5432109876,
					title: "Science Fiction",
					publisher: "Publisher C",
					authors: ["Robert Smith", "Emily Brown"],
					costs: 20,
					year: 2019,
					category: "Science Fiction",
					borrowed: true,
					borrowable: true
				},
				{
					id: "EFGH IJKL 4",
					isbn: 2468135790,
					title: "Mystery Novel",
					publisher: "Publisher D",
					authors: ["David Johnson"],
					costs: 18,
					year: 2021,
					category: "Mystery",
					borrowed: true,
					borrowable: true
				},
				{
					id: "LMNO PQRS 5",
					isbn: 1357924680,
					title: "Fantasy Book",
					publisher: "Publisher D",
					authors: ["Sarah Williams", "Michael Johnson"],
					costs: 12,
					year: 2022,
					category: "Fantasy",
					borrowed: false,
					borrowable: true
				},
				{
					id: "TUVW XYZA 6",
					isbn: 9876543210,
					title: "Adventure Story",
					publisher: "Publisher E",
					authors: ["Lisa Brown", "Mark Davis"],
					costs: 14,
					year: 2020,
					category: "Adventure",
					borrowed: false,
					borrowable: true
				},
				{
					id: "BCDE FGHI 7",
					isbn: 1234567890,
					title: "Historical Fiction",
					publisher: "Publisher F",
					authors: ["Jennifer Smith"],
					costs: 16,
					year: 2019,
					category: "Historical",
					borrowed: true,
					borrowable: true
				},
				{
					id: "JKLM NOPQ 8",
					isbn: 1357924680,
					title: "Biography",
					publisher: "Publisher G",
					authors: ["Alex Johnson", "Emily White"],
					costs: 13,
					year: 2021,
					category: "Biography",
					borrowed: false,
					borrowable: true
				},
				{
					id: "QRST UVWX 9",
					isbn: 9876543210,
					title: "Romance Novel",
					publisher: "Publisher H",
					authors: ["Sophia Miller", "James Brown"],
					costs: 11,
					year: 2020,
					category: "Romance",
					borrowed: false,
					borrowable: true
				},
				{
					id: "YZAB CDEF 10",
					isbn: 1234567890,
					title: "Self-Help Book",
					publisher: "Publisher I",
					authors: ["Emma Davis"],
					costs: 17,
					year: 2022,
					category: "Self-Help",
					borrowed: true,
					borrowable: true
				},
				{
					id: "GHIJ KLMN 11",
					isbn: 1357924680,
					title: "Cookbook",
					publisher: "Publisher J",
					authors: ["Michael Wilson", "Linda Jackson"],
					costs: 15,
					year: 2021,
					category: "Cooking",
					borrowed: false,
					borrowable: true
				},
				{
					id: "OPQR STUV 12",
					isbn: 9876543210,
					title: "Science Book",
					publisher: "Publisher K",
					authors: ["Peter Johnson", "Karen White"],
					costs: 22,
					year: 2019,
					category: "Science",
					borrowed: false,
					borrowable: false
				},
				{
					id: "WXYZ ABCD 13",
					isbn: 1234567890,
					title: "Business Guide",
					publisher: "Publisher L",
					authors: ["Andrew Davis", "Mary Smith"],
					costs: 19,
					year: 2020,
					category: "Business",
					borrowed: false,
					borrowable: true
				},
				{
					id: "EFGH IJKL 14",
					isbn: 1357924680,
					title: "Travelogue",
					publisher: "Publisher M",
					authors: ["Olivia Johnson", "Robert Brown"],
					costs: 16,
					year: 2021,
					category: "Travel",
					borrowed: true,
					borrowable: true
				},
				{
					id: "MNOP QRST 15",
					isbn: 9876543210,
					title: "Psychology Book",
					publisher: "Publisher N",
					authors: ["Sophie White", "Matthew Davis"],
					costs: 14,
					year: 2019,
					category: "Psychology",
					borrowed: false,
					borrowable: true
				},
				{
					id: "UVWX YZAB 16",
					isbn: 1234567890,
					title: "Art History",
					publisher: "Publisher O",
					authors: ["David Smith"],
					costs: 20,
					year: 2020,
					category: "Art",
					borrowed: false,
					borrowable: true
				},
				{
					id: "CDEF GHJI 17",
					isbn: 1357924680,
					title: "Political Science",
					publisher: "Publisher P",
					authors: ["Jennifer White", "Mark Johnson"],
					costs: 18,
					year: 2021,
					category: "Politics",
					borrowed: true,
					borrowable: true
				},
				{
					id: "KLMO PQRS 18",
					isbn: 9876543210,
					title: "Health and Wellness",
					publisher: "Publisher Q",
					authors: ["Emily Davis", "William Brown"],
					costs: 12,
					year: 2022,
					category: "Health",
					borrowed: false,
					borrowable: true
				},
				{
					id: "TUVW XYZA 19",
					isbn: 1234567890,
					title: "Technology Guide",
					publisher: "Publisher R",
					authors: ["Michael Johnson"],
					costs: 21,
					year: 2021,
					category: "Technology",
					borrowed: true,
					borrowable: true
				},
				{
					id: "BCDE FGHI 20",
					isbn: 1357924680,
					title: "Nature Encyclopedia",
					publisher: "Publisher S",
					authors: ["Sophia Davis", "Robert Smith"],
					costs: 15,
					year: 2020,
					category: "Nature",
					borrowed: false,
					borrowable: false
				}
			]}
		/>
	</span>
	<BookView slot="view" bind:book={active} bind:isNew />
</Container>
