<script lang="ts">
	import { _ } from "svelte-i18n";
	import Dialog from "./Dialog.svelte";

	let dialog: Dialog;
	let al: any;

	export async function request(
		url: string,
		type: string,
		json: BodyInit | null | undefined
	): Promise<any> {
		try {
			const response = await fetch(url, {
				method: type,
				headers: {
					"Content-Type": "application/json; charset=utf-8"
				},
				body: json
			});

			if (response.status === 200) {
				const contentType = response.headers.get("Content-Type");

				if (contentType && contentType.includes("application/json")) {
					return response.json();
				} else {
					return response.text();
				}
			} else {
				const data = await response.text();
				throw data;
			}
		} catch (error) {
			if (dialog) dialog.open();
			al = error as string;
			throw error;
		}
	}
</script>

<Dialog bind:this={dialog} min={"fit"} size={"small"}>
	<h5 slot="header" class="m-0">{$_(".alert.error")}</h5>
	<p slot="body" class="m-0 fs-6">{$_(al)}</p>
</Dialog>
