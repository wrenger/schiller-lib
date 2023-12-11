<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { DateTime } from 'luxon';
	import { settingsGlobal } from '$lib/store';
	import { onMount } from 'svelte';
	import {
		getModalStore,
		getToastStore,
		type ModalSettings,
		type ToastSettings
	} from '@skeletonlabs/skeleton';

	const modalStore = getModalStore();
	const toastStore = getToastStore();

	let mail_last_reminder: DateTime = DateTime.fromISO('');

	settingsGlobal.subscribe((s) => {
		mail_last_reminder = s.mail_last_reminder;
	});

	let mounted = false;

	onMount(() => (mounted = true));

	$: if (
		mounted &&
		mail_last_reminder.isValid &&
		Math.ceil(mail_last_reminder.diffNow('days').days) < 0
	) {
		const modal: ModalSettings = {
			type: 'component',
			component: 'remindersModal'
		};
		modalStore.clear();
		modalStore.trigger(modal);
	} else if (!mail_last_reminder.isValid) {
		const t: ToastSettings = {
			message: $_('.error.date'),
			background: 'variant-filled-error'
		};
		toastStore.trigger(t);
	}
</script>
