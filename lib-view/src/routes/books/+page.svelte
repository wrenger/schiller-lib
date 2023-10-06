<script>
	import BookList from "./BookList.svelte";
	import BookSearch from "./BookSearch.svelte";
</script>

<svelte:head>
	<title>Books</title>
	<meta name="description" content="Shows the Books" />
</svelte:head>

<BookSearch />
<BookList
	items={[
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		},
		{
			id: "XXXX XXXX 1",
			isbn: 2390489023,
			title: "Test",
			publisher: "Klett",
			authors: ["Lars", "Nils"],
			costs: 5,
			year: 2023,
			category: "",
			borrowed: false,
			borrowable: true
		},
		{
			id: "ffff dddd 2",
			isbn: 2390848202,
			title: "Langweilig",
			publisher: "Nichts",
			authors: ["Lars"],
			costs: 1,
			year: 2022,
			category: "",
			borrowed: false,
			borrowable: false
		},
		{
			id: "cccc eeee 1",
			isbn: 4958395809,
			title: "Krass",
			publisher: "Maaan",
			authors: ["Nils"],
			costs: 500,
			year: 2023,
			category: "",
			borrowed: true,
			borrowable: true
		}
	]}
/>
