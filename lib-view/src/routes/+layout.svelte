<script lang="ts" context="module">
	import { _ } from "svelte-i18n";
	import Dialog from "../components/basic/Dialog.svelte";

	export async function request(
		url: string,
		type: string,
		json: BodyInit | null | undefined
	): Promise<any> {
		const response = await fetch(url, {
			method: type,
			body: json
		});

		let data = await response.json();

		if (response.status === 200) {
			return data;
		} else {
			error(data);
		}
	}

	export function error(data: string) {
		let formattedError = _.subscribe((_) => {
			return _(data);
		}).toString();
		let errorDialog = document.getElementById("error-dialog") as Dialog | null;
		if (errorDialog) errorDialog.open("Fehler", formattedError);
		throw formattedError;
	}
</script>

<script lang="ts">
	import "../bootstrap/app.scss";
	import "../bootstrap/main.ts";
	import Navbar from "../components/basic/Navbar.svelte";
</script>

<div class="app">
	<Navbar />
	<main>
		<slot />
		<Dialog id={"error-dialog"} />
	</main>
</div>
