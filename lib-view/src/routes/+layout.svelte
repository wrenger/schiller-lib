<script lang="ts">
	import { _ } from "svelte-i18n";
	import { state } from "$lib/store";
	import "../bootstrap/app.scss";
	import "../bootstrap/main.ts";
	import Navbar from "../components/basic/Navbar.svelte";
	import Dialog from "../components/basic/Dialog.svelte";
</script>

<div class="app">
	<Navbar />
	{#key $state}
		<main>
			<slot />
		</main>
	{/key}
</div>

<Dialog id="error-modal" min={"fit"} size={"small"}>
	<h5 slot="header" class="m-0">{$_(".alert.error")}</h5>
	<span slot="body">
		<p class="m-0 fs-6" />
	</span>
</Dialog>
