<script>
	import "../bootstrap/app.scss";
	import "../bootstrap/main.ts";
	import Navbar from "../components/basic/Navbar.svelte";
</script>

<div class="app">
	<Navbar />
	<main>
		<slot />
	</main>
</div>