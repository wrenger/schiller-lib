<script lang="ts">
	import { _ } from 'svelte-i18n';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { settingsGlobal } from '$lib/store';
	import { toast } from 'svelte-sonner';
	import api from '$lib/api';
	import { DateTime } from 'luxon';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { onOutsideClick, mail_replace, handle_result } from '$lib';
	import { Mail } from 'lucide-svelte';

	let open = false;
	let opened = false;
	let overdues: api.Overdue[] = [];

	settingsGlobal.subscribe(async (settings) => {
		if (
			!opened &&
			settings.mail_last_reminder.isValid &&
			Math.ceil(settings.mail_last_reminder.diffNow('days').days) < 0
		) {
			overdues = handle_result(await api.lending_overdues());
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
		let dataToSend: api.Message[] = [];

		for (const { book, user } of overdues) {
			let borrower = book.borrower;
			if (borrower != null) {
				let deadline = DateTime.fromISO(borrower.deadline);
				let template =
					-deadline.diffNow('days').days > $settingsGlobal.overdue_warning_delay
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
			<AlertDialog.Title>{$_('.alert.overdue')}</AlertDialog.Title>
			<AlertDialog.Description>
				{$_('.alert.mail.overdue', { values: { '0': overdues.length } })}
				<ul class="max-h-40 list-inside list-disc overflow-y-auto text-left">
					{#each overdues as overdue}
						<li>{overdue.user.forename} {overdue.user.surname} - {overdue.book.title}</li>
					{/each}
				</ul>
			</AlertDialog.Description>
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
				<Mail class="ml-2 h-4 w-4" />
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
