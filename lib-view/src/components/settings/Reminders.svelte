<script lang="ts">
	import { _ } from "svelte-i18n";
	import { DateTime } from "luxon";
	import Dialog from "../basic/Dialog.svelte";
	import { settingsGlobal, state } from "$lib/store";
	import Spinner from "../basic/Spinner.svelte";
	import { onMount } from "svelte";
	import api from "$lib/api";

	let mail_last_reminder: DateTime = DateTime.fromISO("");

	settingsGlobal.subscribe((s) => {
		mail_last_reminder = s.mail_last_reminder;
	});

	let remDialog: Dialog;
	let errDialog: Dialog;
	let mounted = false;

	onMount(() => (mounted = true));

	$: if (
		mounted &&
		mail_last_reminder.isValid &&
		Math.ceil(mail_last_reminder.diffNow("days").days) < 0
	) {
		remDialog.open();
	} else if (!mail_last_reminder.isValid) {
		errDialog.open();
	}

	let remResponse: Promise<void>;
	async function sendReminders() {
		let overdoneBooks: [api.Book, api.User][] = await api.overdues();

		let dataToSend: api.MailBody[] = [];

		for (const [book, user] of overdoneBooks) {
			if (-DateTime.fromISO(book.deadline ? book.deadline : "").diffNow("days").days > 14) {
				dataToSend.push({
					account: book.borrower || "",
					subject: $settingsGlobal.mail_overdue2_subject
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
					body: $settingsGlobal.mail_overdue2_content
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
				});
			} else {
				dataToSend.push({
					account: book.borrower || "",
					subject: $settingsGlobal.mail_overdue_subject
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
					body: $settingsGlobal.mail_overdue_content
						.replace(/\{booktitle\}/g, book.title)
						.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
				});
			}
		}

		await api.mail(dataToSend);

		let data = await api.settings();

		settingsGlobal.set({ ...data, mail_last_reminder: DateTime.fromISO(data.mail_last_reminder) });

		mail_last_reminder = DateTime.now();

		await api.settings_update({
			...$settingsGlobal,
			mail_last_reminder: mail_last_reminder.toISODate() || ""
		});

		settingsGlobal.set({
			...$settingsGlobal,
			mail_last_reminder
		});

		state.set({});

		remDialog.close();
	}
</script>

<Dialog bind:this={remDialog} size="small" min="fit">
	<span slot="header"><h5 class="mb-0">{$_(".alert.confirm")}</h5></span>
	<span slot="body"><p class="m-0 fs-6">{$_(".alert.mail.overdue")}</p></span>
	<span slot="footer">
		<button type="button" class="btn btn-primary" on:click={() => (remResponse = sendReminders())}>
			<Spinner response={remResponse} />
			{$_(".action.ok")}
		</button>
	</span>
</Dialog>

<Dialog bind:this={errDialog} size="small" min="fit">
	<h5 slot="header" class="mb-0">{$_(".alert.error")}</h5>
	<span slot="body"><p class="m-0 fs-6">{$_(".error.date")}</p></span>
</Dialog>
