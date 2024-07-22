<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { settingsGlobal } from '$lib/store';
	import { DateTime } from 'luxon';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let open = false;
	let response: Promise<any>;

	async function return_back() {
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

			await api.mail([{ account: book.reservation, ...mail }]);

			await return_back();
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
			<Dialog.Description>
				{$_('.book.borrowed.by', {
					values: {
						'0': book?.borrower?.user,
						'1': book?.borrower && DateTime.fromISO(book.borrower.deadline).toLocaleString()
					}
				})}
			</Dialog.Description>
		</Dialog.Header>
		<hr />
		<div>
			<span class="text-muted-foreground text-sm">{$_('.book.note')}:</span>
			<span class="text-md whitespace-pre-line font-medium"
				>{book?.note || $_('.action.empty')}</span
			>
		</div>
		{#if book?.reservation}
			<hr />
			<div class="whitespace-pre-line">
				{$_('.book.revoke.reminder', { values: { '0': book.reservation } })}
			</div>
		{/if}
		<Dialog.Footer>
			{#if book?.reservation}
				<Button variant="secondary" on:click={() => (response = return_back())}>
					<Spinner {response} />
					{$_('.action.no')}
				</Button>
				<Button on:click={() => (response = mail())}>
					<Spinner {response} />
					{$_('.action.yes')}
				</Button>
			{:else}
				<Button on:click={() => (response = return_back())}>
					<Spinner {response} />
					{$_('.book.revoke')}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
