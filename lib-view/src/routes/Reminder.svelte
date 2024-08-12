<script lang="ts">
	import { _ } from 'svelte-i18n';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { settingsGlobal } from '$lib/store';
	import { toast } from 'svelte-sonner';
	import api from '$lib/api';
	import { DateTime } from 'luxon';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { onOutsideClick, mail_replace, handle_result } from '$lib';

	let open = false;
	let opened = false;

	settingsGlobal.subscribe(async (settings) => {
		if (
			!opened &&
			settings.mail_last_reminder.isValid &&
			Math.ceil(settings.mail_last_reminder.diffNow('days').days) < 0
		) {
			let overdues = handle_result(await api.lending_overdues());
			if (overdues.length > 0) {
				open = true;
				opened = true;
			}
		} else if (!settings.mail_last_reminder.isValid) {
			toast.error($_('.error.date'));
		}
	});

	let response: Promise<void>;
	async function sendReminders() {
		let overdueBooks = handle_result(await api.lending_overdues());
		let dataToSend: api.Message[] = [];

		for (const { book, user } of overdueBooks) {
			let borrower = book.borrower;
			if (borrower != null) {
				let template =
					-DateTime.fromISO(borrower.deadline).diffNow('days').days > 14
						? $settingsGlobal.mail_overdue2
						: $settingsGlobal.mail_overdue;
				let mail = mail_replace(template, book.title, `${user.forename} ${user.surname}`);
				dataToSend.push({ account: borrower.user, ...mail });
			} else {
				console.error('No borrower found for book', book);
			}
		}

		handle_result(await api.mail_notify(dataToSend));

		let mail_last_reminder = DateTime.now().toISODate();

		handle_result(
			await api.settings_update({
				...$settingsGlobal,
				mail_last_reminder
			})
		);

		settingsGlobal.set({
			...$settingsGlobal,
			mail_last_reminder: DateTime.fromISO(mail_last_reminder)
		});

		open = false;
	}
</script>

<AlertDialog.Root bind:open {onOutsideClick}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{$_('.alert.confirm')}</AlertDialog.Title>
			<AlertDialog.Description>{$_('.alert.mail.overdue')}</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>{$_('.action.close')}</AlertDialog.Cancel>
			<AlertDialog.Action
				on:click={(event) => {
					response = sendReminders();
					event.preventDefault();
				}}
			>
				<Spinner {response} />
				{$_('.action.ok')}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
