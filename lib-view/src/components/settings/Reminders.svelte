<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { settingsGlobal } from '$lib/store';
	import {
		getModalStore,
		getToastStore,
		type ModalSettings,
		type ToastSettings
	} from '@skeletonlabs/skeleton';

	const modalStore = getModalStore();
	const toastStore = getToastStore();

	settingsGlobal.subscribe((settings) => {
		if (
			settings.mail_last_reminder.isValid &&
			Math.ceil(settings.mail_last_reminder.diffNow('days').days) < 0
		) {
			const modal: ModalSettings = {
				type: 'component',
				component: 'remindersModal'
			};
			modalStore.clear();
			modalStore.trigger(modal);
		} else if (!settings.mail_last_reminder.isValid) {
			const t: ToastSettings = {
				message: $_('.error.date'),
				background: 'variant-filled-error'
			};
			toastStore.trigger(t);
		}
	});
</script>
