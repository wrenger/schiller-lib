<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { settingsGlobal } from '$lib/store';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let open = false;

	let response1: Promise<any>;
	let response2: Promise<any>;
	async function ret() {
		if (book) {
			book = await api.return_back(book.id);
			open = false;
			onChange(book);
		}
	}

	async function mail() {
		if (book && book.reservation) {
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

			await ret();
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={(value) => (open = value)} {onOutsideClick}>
	<Dialog.Trigger asChild let:builder={dialog}>
		<slot {dialog} />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>
				{$_('.book.revoke')}
			</Dialog.Title>
			{#if book?.reservation}
				<Dialog.Description>
					{$_('.book.revoke.reminder', { values: { '0': book.reservation } })}
				</Dialog.Description>
			{/if}
		</Dialog.Header>
		<Dialog.Footer>
			{#if book?.reservation}
				<Button variant="outline" on:click={() => (response1 = ret())}>
					<Spinner response={response1} />
					{$_('.action.no')}
				</Button>
				<Button on:click={() => (response2 = mail())}>
					<Spinner response={response2} />
					{$_('.action.yes')}
				</Button>
			{:else}
				<Button on:click={() => (response1 = ret())}>
					<Spinner response={response1} />
					{$_('.action.ok')}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
