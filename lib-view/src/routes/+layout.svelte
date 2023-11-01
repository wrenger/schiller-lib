<script lang="ts">
	import { category, settingsGlobal } from "$lib/store";
	import "../bootstrap/app.scss";
	import "../bootstrap/main.ts";
	import Navbar from "../components/basic/Navbar.svelte";
</script>

<div class="app">
	<Navbar />
	{#key $settingsGlobal || $category}
		<main>
			<slot />
		</main>
	{/key}
</div>
