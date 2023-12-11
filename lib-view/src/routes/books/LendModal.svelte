<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { DateTime } from 'luxon';
	// import Dialog from "../../components/basic/Dialog.svelte";
	import { settingsGlobal } from '$lib/store';
	import UserSelect from '../users/UserSelect.svelte';
	import DateField from '../../components/basic/DateField.svelte';
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

	let period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
	let borrower = $modalStore[0].meta.borrower;
	let response: Promise<void>;

	async function lend() {
		let book = await api.lend($modalStore[0].meta.bookId, borrower, period?.toISODate());
		$modalStore[0].meta.onChange(book);
		modalStore.close();
	}
</script>

<!-- @component This is a Lend Modal. -->

{#if $modalStore[0]}
	<div class={cBase}>
		<header class={cHeader}>{$_('.book.lend')}</header>
		<UserSelect label={$_('.user')} placeholder={$_('.user.account')} bind:value={borrower} />
		<DateField bind:date={period} label={$_('.book.lend.period')} />
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{$_(".action.close")}</button>
        <button class="btn {parent.buttonPositive}" on:click={async () => response = lend()}><Spinner {response} />
			{$_(".action.apply")}</button>
    </footer>
	</div>
{/if}
