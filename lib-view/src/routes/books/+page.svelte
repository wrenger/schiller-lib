<script lang="ts">
	import { _ } from "svelte-i18n";
	import Container from "../../components/basic/Container.svelte";
	import BookList from "./BookList.svelte";
	import BookSearch, { BookParams } from "./BookSearch.svelte";
	import BookView, { Book } from "./BookView.svelte";
	let params: BookParams;
	let active: Book | null;
	let isNew: boolean;
</script>

<svelte:head>
	<title>{$_(".book")}</title>
	<meta name="description" content={$_(".book")} />
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
					isbn: "1234567890",
					title: "Sample Book 1",
					publisher: "Publisher A",
					authors: ["John Doe", "Jane Smith"],
					costs: 10,
					year: 2021,
					category: "Fiction",
					borrowable: true,
					borrower: "nils.wrenger",
					deadline: "15.04.2023",
					reservation: undefined
				},
				{
					id: "WXYZ 1234 2",
					isbn: "9876543210",
					title: "Another Book",
					publisher: "Publisher B",
					authors: ["Alice Johnson"],
					costs: 15,
					year: 2020,
					category: "Non-Fiction",
					borrowable: true,
					borrower: undefined,
					deadline: undefined,
					reservation: undefined
				},
				{
					id: "QWER TYUI 3",
					isbn: "5432109876",
					title: "Science Fiction",
					publisher: "Publisher C",
					authors: ["Robert Smith", "Emily Brown"],
					costs: 20,
					year: 2019,
					category: "Science Fiction",
					borrowable: false,
					borrower: undefined,
					deadline: undefined,
					reservation: undefined
				},
				{
					id: "EFGH IJKL 4",
					isbn: "2468135790",
					title: "Mystery Novel",
					publisher: "Publisher D",
					authors: ["David Johnson"],
					costs: 18,
					year: 2021,
					category: "Mystery",
					borrowable: true,
					borrower: "emma.johnson",
					deadline: "12.04.2023",
					reservation: "james.brown"
				},
				{
					id: "LMNO PQRS 5",
					isbn: "1357924680",
					title: "Fantasy Book",
					publisher: "Publisher D",
					authors: ["Sarah Williams", "Michael Johnson"],
					costs: 12,
					year: 2022,
					category: "Fantasy",
					borrowable: true,
					borrower: undefined,
					deadline: undefined,
					reservation: "nils.wrenger"
				},
				{
					id: "TUVW XYZA 6",
					isbn: "6789012345",
					title: "Thriller",
					publisher: "Publisher E",
					authors: ["Sophia Davis"],
					costs: 13,
					year: 2020,
					category: "Thriller",
					borrowable: true,
					borrower: "david.wilson",
					deadline: "22.04.2023",
					reservation: undefined
				},
				{
					id: "BCDE FGHI 7",
					isbn: "9876543210",
					title: "Classic Literature",
					publisher: "Publisher F",
					authors: ["Mark Johnson"],
					costs: 15,
					year: 2018,
					category: "Classic",
					borrowable: false,
					borrower: undefined,
					deadline: undefined,
					reservation: undefined
				},
				{
					id: "IJKL MNOP 8",
					isbn: "5432109876",
					title: "Biography",
					publisher: "Publisher G",
					authors: ["Karen Smith", "Daniel Brown"],
					costs: 18,
					year: 2022,
					category: "Biography",
					borrowable: true,
					borrower: "sarah.davis",
					deadline: "10.06.2023",
					reservation: undefined
				},
				{
					id: "PQRS TUVW 9",
					isbn: "1357924680",
					title: "Self-Help",
					publisher: "Publisher H",
					authors: ["Jennifer Davis"],
					costs: 12,
					year: 2021,
					category: "Self-Help",
					borrowable: true,
					borrower: "michael.white",
					deadline: "15.05.2023",
					reservation: undefined
				},
				{
					id: "XYZA BCDE 10",
					isbn: "1234567890",
					title: "Romance Novel",
					publisher: "Publisher I",
					authors: ["Linda Jackson", "Peter Johnson"],
					costs: 10,
					year: 2020,
					category: "Romance",
					borrowable: true,
					borrower: "linda.jackson",
					deadline: "18.04.2023",
					reservation: "mary.smith"
				},
				{
					id: "FGHI JKLM 11",
					isbn: "2468135790",
					title: "Historical Fiction",
					publisher: "Publisher J",
					authors: ["Robert Brown"],
					costs: 18,
					year: 2019,
					category: "Historical Fiction",
					borrowable: true,
					borrower: "robert.brown",
					deadline: "25.04.2023",
					reservation: undefined
				},
				{
					id: "MNOP QRST 12",
					isbn: "6789012345",
					title: "Cookbook",
					publisher: "Publisher K",
					authors: ["Karen White"],
					costs: 13,
					year: 2021,
					category: "Cooking",
					borrowable: true,
					borrower: "karen.white",
					deadline: "20.05.2023",
					reservation: undefined
				},
				{
					id: "UVWX YZAB 13",
					isbn: "9876543210",
					title: "Travel Guide",
					publisher: "Publisher L",
					authors: ["Emily Davis", "William Brown"],
					costs: 12,
					year: 2022,
					category: "Travel",
					borrowable: true,
					borrower: "emily.davis",
					deadline: "15.04.2023",
					reservation: undefined
				},
				{
					id: "CDEF GHIJ 14",
					isbn: "5432109876",
					title: "Art History",
					publisher: "Publisher M",
					authors: ["Sophia Davis", "Robert Smith"],
					costs: 15,
					year: 2020,
					category: "Art",
					borrowable: true,
					borrower: "james.brown",
					deadline: "10.05.2023",
					reservation: undefined
				},
				{
					id: "KLMN OPQR 15",
					isbn: "1357924680",
					title: "Psychology",
					publisher: "Publisher N",
					authors: ["David Wilson", "Sarah Davis"],
					costs: 20,
					year: 2019,
					category: "Psychology",
					borrowable: true,
					borrower: "olivia.jones",
					deadline: "12.04.2023",
					reservation: undefined
				},
				{
					id: "STUV WXYZ 16",
					isbn: "2468135790",
					title: "Science Book",
					publisher: "Publisher O",
					authors: ["Michael Johnson"],
					costs: 18,
					year: 2021,
					category: "Science",
					borrowable: true,
					borrower: "david.wilson",
					deadline: "05.06.2023",
					reservation: "sophie.miller"
				},
				{
					id: "YZAB CDEF 17",
					isbn: "1234567890",
					title: "Drama",
					publisher: "Publisher P",
					authors: ["Sarah Williams", "Karen Smith"],
					costs: 10,
					year: 2022,
					category: "Drama",
					borrowable: true,
					borrower: "michael.white",
					deadline: "22.04.2023",
					reservation: undefined
				},
				{
					id: "GHIJ KLMN 18",
					isbn: "9876543210",
					title: "Business",
					publisher: "Publisher Q",
					authors: ["Peter Johnson", "Linda Jackson"],
					costs: 15,
					year: 2021,
					category: "Business",
					borrowable: true,
					borrower: "linda.jackson",
					deadline: "10.06.2023",
					reservation: undefined
				},
				{
					id: "OPQR STUV 19",
					isbn: "5432109876",
					title: "Technology",
					publisher: "Publisher R",
					authors: ["Robert Brown", "Jennifer Davis"],
					costs: 18,
					year: 2020,
					category: "Technology",
					borrowable: true,
					borrower: "robert.brown",
					deadline: "15.05.2023",
					reservation: undefined
				},
				{
					id: "WXYZ ABCD 20",
					isbn: "1357924680",
					title: "Health",
					publisher: "Publisher S",
					authors: ["Karen White", "Emily Davis"],
					costs: 12,
					year: 2018,
					category: "Health",
					borrowable: false,
					borrower: undefined,
					deadline: undefined,
					reservation: undefined
				}
			]}
		/>
	</span>
	<BookView slot="view" bind:book={active} bind:isNew />
</Container>
