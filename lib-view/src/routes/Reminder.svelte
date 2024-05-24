<script lang="ts">
	import { _ } from 'svelte-i18n';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { settingsGlobal } from '$lib/store';
	import { toast } from 'svelte-sonner';
	import api from '$lib/api';
	import { DateTime } from 'luxon';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { onOutsideClick } from '$lib';

	let open = false;
	let opened = false;

	settingsGlobal.subscribe(async (settings) => {
		if (
			!opened &&
			settings.mail_last_reminder.isValid &&
			Math.ceil(settings.mail_last_reminder.diffNow('days').days) < 0
		) {
			let overdues = await api.overdues();
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
		let overdoneBooks: api.Overdue[] = await api.overdues();

		let dataToSend: api.MailBody[] = [];

		for (const { book, user } of overdoneBooks) {
			let borrower = book.borrower;
			if (borrower != null) {
				let template =
					-DateTime.fromISO(borrower.deadline).diffNow('days').days > 14
						? $settingsGlobal.mail_overdue2
						: $settingsGlobal.mail_overdue;
				let mail = api.mail_replace(template, book.title, `${user.forename} ${user.surname}`);
				dataToSend.push({ account: borrower.user, ...mail });
			} else {
				console.error('No borrower found for book', book);
			}
		}

		await api.mail(dataToSend);

		let mail_last_reminder = DateTime.now().toISODate();

		await api.settings_update({
			...$settingsGlobal,
			mail_last_reminder
		});

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
