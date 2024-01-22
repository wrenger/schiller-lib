<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { DateTime } from 'luxon';
	// import Dialog from "../../components/basic/Dialog.svelte";
	import { settingsGlobal } from '$lib/store';
	import Spinner from '../../components/basic/Spinner.svelte';
	import api from '$lib/api';
	import type { SvelteComponent } from 'svelte';
	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();
	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';

	let mail_last_reminder: DateTime = DateTime.fromISO('');

	settingsGlobal.subscribe((s) => {
		mail_last_reminder = s.mail_last_reminder;
	});

	let response: Promise<void>;
	async function sendReminders() {
		let overdoneBooks: api.Overdue[] = await api.overdues();

		let dataToSend: api.MailBody[] = [];

		for (const {book, user} of overdoneBooks) {
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

		let data = await api.settings();

		settingsGlobal.set({ ...data, mail_last_reminder: DateTime.fromISO(data.mail_last_reminder) });

		mail_last_reminder = DateTime.now();

		await api.settings_update({
			...$settingsGlobal,
			mail_last_reminder: mail_last_reminder.toISODate() || ''
		});

		settingsGlobal.set({
			...$settingsGlobal,
			mail_last_reminder
		});

		modalStore.close();
	}
</script>

<!-- @component This is a Lend Modal. -->

{#if $modalStore[0]}
	<div class={cBase}>
		<header class={cHeader}>{$_('.alert.confirm')}</header>
		<p>{$_('.alert.mail.overdue')}</p>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{$_(".action.close")}</button>
        <button class="btn {parent.buttonPositive}" on:click={async () => response = sendReminders()}><Spinner {response} />
			{$_(".action.ok")}</button>
    </footer>
	</div>
{/if}
