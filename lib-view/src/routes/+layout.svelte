<script lang="ts">
	// css
	import '@fortawesome/fontawesome-free/css/all.css';
	import '../app.postcss';

	// app
	import {
		AppShell,
		AppRail,
		AppRailAnchor,
		Modal,
		getModalStore,
		type ModalSettings,
		type ModalComponent,
		TabGroup,
		TabAnchor,
		setInitialClassState,
		Toast,
		type ToastSettings,
		getToastStore
	} from '@skeletonlabs/skeleton';

	// stores
	import { initializeStores } from '@skeletonlabs/skeleton';
	initializeStores();
	const modalStore = getModalStore();

	const modalRegistry: Record<string, ModalComponent> = {
		settingsModal: { ref: SettingsModal },
		lendModal: { ref: LendModal },
		mailModal: { ref: MailModal },
		reserveModal: { ref: ReserveModal },
		remindersModal: { ref: RemidersModal }
	};

	const toastStore = getToastStore();

	// language
	import { _ } from 'svelte-i18n';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import SettingsModal from '../components/settings/SettingsModal.svelte';
	import api from '../lib/api';
	import { DateTime } from 'luxon';
	import { category, errorStore, settingsGlobal, state } from '$lib/store';
	import { onMount } from 'svelte';
	import LendModal from './books/LendModal.svelte';
	import MailModal from './books/MailModal.svelte';
	import ReserveModal from './books/ReserveModal.svelte';
	import RemidersModal from '../components/settings/RemidersModal.svelte';
	import Reminders from '../components/settings/Reminders.svelte';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	function settings(): void {
		const modal: ModalSettings = {
			type: 'component',
			component: 'settingsModal'
		};
		modalStore.trigger(modal);
	}

	// get info
	async function update() {
		// get settings
		let data = await api.settings();
		let settings = { ...data, mail_last_reminder: DateTime.fromISO(data.mail_last_reminder) };

		settingsGlobal.set(settings);

		// get categories
		let categories = await api.categories();
		category.set(categories);
	}

	async function updatePeriodically() {
		await update();
	}

	onMount(() => {
		// Run the `update` function immediately on mount
		updatePeriodically();

		const interval = setInterval(updatePeriodically, 300000);

		// Cleanup the interval when the component is unmounted
		return () => {
			clearInterval(interval);
		};
	});

	// api errors
	errorStore.subscribe((s) => {
		if (s.message) {
			const t: ToastSettings = {
				message: s.message,
				background: 'variant-filled-error'
			};
			toastStore.trigger(t);
		}
	});
</script>

<svelte:head>{@html `<script>(${setInitialClassState.toString()})();</script>`}</svelte:head>

<Toast position="br" zIndex="z-[1000]" />
<Modal components={modalRegistry} />
<Reminders />

<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<AppRail class="hidden sm:grid">
			<AppRailAnchor
				title={$_('.search.book')}
				href="/books"
				selected={$page.url.pathname === '/books'}
				><svelte:fragment slot="lead"><i class="fa-solid fa-book"></i></svelte:fragment>
				<span class="truncate w-16 inline-block">{$_('.search.book')}</span></AppRailAnchor
			>
			<AppRailAnchor
				title={$_('.search.user')}
				href="/users"
				selected={$page.url.pathname === '/users'}
				><svelte:fragment slot="lead"><i class="fa-solid fa-user-group"></i></svelte:fragment>
				<span class="truncate w-16 inline-block">{$_('.search.user')}</span></AppRailAnchor
			>
			<AppRailAnchor
				title={$_('.book.overdues')}
				href="/overdues"
				selected={$page.url.pathname === '/overdues'}
				><svelte:fragment slot="lead"><i class="fa-solid fa-clock"></i></svelte:fragment>
				<span class="truncate w-16 inline-block">{$_('.book.overdues')}</span></AppRailAnchor
			>
			<AppRailAnchor
				title={$_('.alert.info')}
				href="/info"
				selected={$page.url.pathname === '/info'}
				><svelte:fragment slot="lead"><i class="fa-solid fa-circle-info"></i></svelte:fragment>
				<span class="truncate w-16 inline-block">{$_('.alert.info')}</span></AppRailAnchor
			>
			<svelte:fragment slot="trail">
				<AppRailAnchor
					title={$_('.action.logout')}
					href="/auth/logout"
					selected={$page.url.pathname === '/auth/logout'}
					><svelte:fragment slot="lead"
						><i class="fa-solid fa-right-from-bracket"></i>
					</svelte:fragment>
					<span class="truncate w-16 inline-block">{$_('.action.logout')}</span>
				</AppRailAnchor>
				<AppRailAnchor title={$_('.pref.title')} on:click={settings} style="user-select: none;"
					><svelte:fragment slot="lead"><i class="fa-solid fa-gear"></i></svelte:fragment>
					<span class="truncate w-16 inline-block">{$_('.pref.title')}</span></AppRailAnchor
				>
			</svelte:fragment>
		</AppRail>
	</svelte:fragment>
	<svelte:fragment slot="footer">
		<TabGroup
			justify="justify-center"
			active="variant-filled-primary"
			hover="hover:variant-soft-primary"
			flex="flex-1 lg:flex-none"
			rounded=""
			border=""
			class="bg-surface-100-800-token w-full sm:hidden"
		>
			<TabAnchor
				title={$_('.search.book')}
				href="/books"
				selected={$page.url.pathname === '/books'}
			>
				<svelte:fragment slot="lead"><i class="fa-solid fa-book"></i></svelte:fragment>
			</TabAnchor>
			<TabAnchor
				title={$_('.search.user')}
				href="/users"
				selected={$page.url.pathname === '/users'}
			>
				<svelte:fragment slot="lead"><i class="fa-solid fa-user-group"></i></svelte:fragment>
			</TabAnchor>
			<TabAnchor
				title={$_('.book.overdues')}
				href="/overdues"
				selected={$page.url.pathname === '/overdues'}
			>
				<svelte:fragment slot="lead"><i class="fa-solid fa-clock"></i></svelte:fragment>
			</TabAnchor>
			<TabAnchor title={$_('.alert.info')} href="/info" selected={$page.url.pathname === '/info'}>
				<svelte:fragment slot="lead"><i class="fa-solid fa-circle-info"></i></svelte:fragment>
			</TabAnchor>
			<TabAnchor
				title={$_('.action.logout')}
				href="/auth/logout"
				selected={$page.url.pathname === '/auth/logout'}
				><svelte:fragment slot="lead"
					><i class="fa-solid fa-right-from-bracket"></i></svelte:fragment
				>
			</TabAnchor>
			<TabAnchor title={$_('.pref.title')} on:click={settings} style="user-select: none;"
				><svelte:fragment slot="lead"><i class="fa-solid fa-gear"></i></svelte:fragment>
			</TabAnchor>
		</TabGroup>
	</svelte:fragment>
	{#key $state}
		<main class="p-2">
			<slot />
		</main>
	{/key}
</AppShell>
