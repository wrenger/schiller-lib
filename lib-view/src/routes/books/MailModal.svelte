<script lang="ts">
	import { _ } from 'svelte-i18n';
	// import Dialog from "../../components/basic/Dialog.svelte";
	import Spinner from '../../components/basic/Spinner.svelte';
	import api from '$lib/api';
	import type { SvelteComponent } from 'svelte';
	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { settingsGlobal } from '$lib/store';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();
	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';

	let book: api.Book = $modalStore[0].meta.book;
	let response: Promise<void>;

	async function mail() {
		if (book == null || book.reservation == null) return;

		let user = await api.user_fetch(book.reservation);

		let mail = api.mail_replace(
			$settingsGlobal.mail_info,
			book.title,
			`${user.forename} ${user.surname}`
		);

		await api.mail([
			{
				account: book.reservation,
				...mail
			}
		]);
		modalStore.close();
	}
</script>

<!-- @component This is a Reservation Modal. -->

{#if $modalStore[0]}
	<div class={cBase}>
		<header class={cHeader}>{$_('.alert.confirm')}</header>
		<p>
			{$_('.book.revoke.reminder', { values: { '0': book?.reservation } })}
		</p>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{$_(".action.close")}</button>
        <button class="btn {parent.buttonPositive}" on:click={async () => response = mail()}><Spinner {response} />
			{$_(".action.ok")}</button>
    </footer>
	</div>
{/if}
