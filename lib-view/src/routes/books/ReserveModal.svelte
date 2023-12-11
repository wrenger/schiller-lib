<script lang="ts">
	import { _ } from 'svelte-i18n';
	// import Dialog from "../../components/basic/Dialog.svelte";
	import UserSelect from '../users/UserSelect.svelte';
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

	let gonnaReserve: string = $modalStore[0].meta.reservation;
	let response: Promise<void>;

	async function reserve() {
		let book = await api.reserve($modalStore[0].meta.bookId, gonnaReserve);
		$modalStore[0].meta.onChange(book);
		modalStore.close();
	}
</script>

<!-- @component This is a Reservation Modal. -->

{#if $modalStore[0]}
	<div class={cBase}>
		<header class={cHeader}>{$_('.book.lend')}</header>
		<UserSelect label={$_('.user')} placeholder={$_('.user.account')} bind:value={gonnaReserve} />
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{$_(".action.close")}</button>
        <button class="btn {parent.buttonPositive}" on:click={async () => response = reserve()}><Spinner {response} />
			{$_(".action.apply")}</button>
    </footer>
	</div>
{/if}
