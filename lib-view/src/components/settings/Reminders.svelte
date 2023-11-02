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

	let borrowing_duration = 0;
	let dnb_token = "";
	let mail_last_reminder = "";
	let mail_from = "";
	let mail_host = "";
	let mail_password = "";
	let mail_info_subject = "";
	let mail_info_content = "";
	let mail_overdue_subject = "";
	let mail_overdue_content = "";
	let mail_overdue2_subject = "";
	let mail_overdue2_content = "";

	settingsGlobal.subscribe((s) => {
		borrowing_duration = s.borrowing_duration;
		dnb_token = s.dnb_token;
		mail_last_reminder = s.mail_last_reminder;
		mail_from = s.mail_from;
		mail_host = s.mail_host;
		mail_password = s.mail_password;
		mail_info_subject = s.mail_info_subject;
		mail_info_content = s.mail_info_content;
		mail_overdue_subject = s.mail_overdue_subject;
		mail_overdue_content = s.mail_overdue_content;
		mail_overdue2_subject = s.mail_overdue2_subject;
		mail_overdue2_content = s.mail_overdue2_content;
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
	}

	let remResponse: Promise<any>;
	async function sendReminders() {
		let overdoneBooks: [Book, User][] = await r.request("api/overdues", "GET", null);

		overdoneBooks.forEach(async (e: [Book, User]) => {
			let book = e[0];
			let user = e[1];

			if (-DateTime.fromISO(book.deadline ? book.deadline : "").diffNow("days").days > 14) {
				await r.request(
					`/api/notify`,
					"POST",
					JSON.stringify({
						account: book.borrower,
						subject: $settingsGlobal.mail_overdue2_subject
							.replace(/\{booktitle\}/g, book.title)
							.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
						body: $settingsGlobal.mail_overdue2_content
							.replace(/\{booktitle\}/g, book.title)
							.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
					})
				);
			} else {
				await r.request(
					`/api/notify`,
					"POST",
					JSON.stringify({
						account: book.borrower,
						subject: $settingsGlobal.mail_overdue_subject
							.replace(/\{booktitle\}/g, book.title)
							.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : ""),
						body: $settingsGlobal.mail_overdue_content
							.replace(/\{booktitle\}/g, book.title)
							.replace(/\{username\}/g, user ? `${user.forename} ${user.surname}` : "")
					})
				);
			}
		});

		mail_last_reminder = DateTime.now().toISODate() || "";

		await r.request(
			"api/settings",
			"POST",
			JSON.stringify({
				borrowing_duration,
				dnb_token,
				mail_last_reminder,
				mail_from,
				mail_host,
				mail_password,
				mail_info_subject,
				mail_info_content,
				mail_overdue_subject,
				mail_overdue_content,
				mail_overdue2_subject,
				mail_overdue2_content
			})
		);

		settingsGlobal.set({
			borrowing_duration,
			dnb_token,
			mail_last_reminder,
			mail_from,
			mail_host,
			mail_password,
			mail_info_subject,
			mail_info_content,
			mail_overdue_subject,
			mail_overdue_content,
			mail_overdue2_subject,
			mail_overdue2_content
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
