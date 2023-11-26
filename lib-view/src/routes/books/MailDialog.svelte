<script lang="ts">
	import api from "$lib/api";
	import { settingsGlobal } from "$lib/store";
	import Dialog from "../../components/basic/Dialog.svelte";
	import Spinner from "../../components/basic/Spinner.svelte";

	import { _ } from "svelte-i18n";

	let book: api.Book | null = null;

	export function open(b: api.Book) {
		book = b;
		dialog.open();
	}

	export function close() {
		dialog.close();
	}

	let dialog: Dialog;
	let response: Promise<void>;

	async function mail() {
		if (book === null) return;

		let user = await api.user_fetch(book.reservation);

		await api.mail([
			{
				account: book.reservation,
				subject: $settingsGlobal.mail_info_subject
					.replace(/\{booktitle\}/g, book.title)
					.replace(/\{username\}/g, `${user.forename} ${user.surname}`),
				body: $settingsGlobal.mail_info_content
					.replace(/\{booktitle\}/g, book.title)
					.replace(/\{username\}/g, `${user.forename} ${user.surname}`)
			}
		]);
		dialog.close();
	}
</script>

<Dialog bind:this={dialog} size="small" min="fit">
	<span slot="header"><h5 class="mb-0">{$_(".alert.confirm")}</h5></span>
	<span slot="body"
		><p class="m-0 fs-6">
			{$_(".book.revoke.reminder", { values: { "0": book?.reservation } })}
		</p></span
	>
	<span slot="footer">
		<button type="button" class="btn btn-primary" on:click={() => (response = mail())}>
			<Spinner {response} />
			{$_(".action.ok")}
		</button>
	</span>
</Dialog>
