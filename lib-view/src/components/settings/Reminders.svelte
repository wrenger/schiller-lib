<script lang="ts">
	import { _ } from "svelte-i18n";
	import { DateTime } from "luxon";
	import type { Book } from "../../routes/books/BookView.svelte";
	import type { User } from "../../routes/users/UserView.svelte";
	import Request from "../basic/Request.svelte";
	import Dialog from "../basic/Dialog.svelte";
	import { settingsGlobal, state } from "$lib/store";
	import Spinner from "../basic/Spinner.svelte";
	import { onMount } from "svelte";

	let mail_last_reminder = "";

	settingsGlobal.subscribe((s) => {
		mail_last_reminder = s.mail_last_reminder;
	});

	let remDialog: Dialog;
	let mounted = false;
	let r: Request;

	onMount(() => (mounted = true));

	$: if (
		$settingsGlobal.mail_last_reminder &&
		mounted &&
		!DateTime.now().hasSame(DateTime.fromISO($settingsGlobal.mail_last_reminder), "day")
	) {
		remDialog.open();
		mounted = false;
	}

	let remResponse: Promise<any>;
	async function sendReminders() {
		let overdoneBooks: [Book, User][] = await r.request("api/overdues", "GET", null);

		let dataToSend: {}[] = [];

		for (const [book, user] of overdoneBooks) {
			if (-DateTime.fromISO(book.deadline ? book.deadline : "").diffNow("days").days > 14) {
				dataToSend.push({
					account: book.borrower,
					subject: $settingsGlobal.mail_overdue2_subject
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
					body: $settingsGlobal.mail_overdue2_content
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
				});
			} else {
				dataToSend.push({
					account: book.borrower,
					subject: $settingsGlobal.mail_overdue_subject
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
					body: $settingsGlobal.mail_overdue_content
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
				});
			}
		}

		await r.request(`/api/notify`, "POST", JSON.stringify(dataToSend));

		let data = await r.request("api/settings", "GET", null);

		settingsGlobal.set(data);

		mail_last_reminder = DateTime.now().toISODate() || "";

		await r.request(
			"api/settings",
			"POST",
			JSON.stringify({
				...$settingsGlobal,
				mail_last_reminder
			})
		);

		settingsGlobal.set({
			...$settingsGlobal,
			mail_last_reminder
		});

		state.set({});

		remDialog.close();
	}
</script>

<Request bind:this={r} />

<Dialog bind:this={remDialog}>
	<span slot="header"><h5 class="mb-0">{$_(".alert.confirm")}</h5></span>
	<span slot="body">{$_(".alert.mail.overdue")}</span>
	<span slot="footer">
		<button type="button" class="btn btn-primary" on:click={() => (remResponse = sendReminders())}>
			<Spinner response={remResponse} />
			{$_(".action.ok")}
		</button>
	</span>
</Dialog>
