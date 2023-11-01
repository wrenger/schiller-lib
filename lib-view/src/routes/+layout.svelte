<script lang="ts">
	import { state } from "$lib/store";
	import "../bootstrap/app.scss";
	import "../bootstrap/main.ts";
	import Navbar from "../components/basic/Navbar.svelte";
</script>

<div class="app">
	<Navbar />
	{#key $state}
		<main>
			<slot />
		</main>
	{/key}
</div>
