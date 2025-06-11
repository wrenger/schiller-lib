<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { handle_result, mail_replace, onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { settingsGlobal } from '$lib/store';
	import { DateTime } from 'luxon';
	import { Mail } from 'lucide-svelte';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let open = false;
	let response_back: Promise<any>;
	let response_mail: Promise<any>;

	async function return_back() {
		if (book) {
			book = handle_result(await api.lending_return({ id: book.id }));
			open = false;
			onChange(book);
		}
	}

	async function mail() {
		if (book && book.reservation) {
			let user = handle_result(await api.user_fetch(book.reservation));
			let mail = mail_replace(
				$settingsGlobal.mail_info,
				book.title,
				`${user.forename} ${user.surname}`
			);

			handle_result(await api.mail_notify([{ account: book.reservation, ...mail }]));

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
			<span class="text-sm text-muted-foreground">{$_('.book.note')}:</span>
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
				<Button variant="outline" on:click={() => (response_back = return_back())}>
					<Spinner response={response_back} />
					{$_('.action.no')}
				</Button>
				<Button on:click={() => (response_mail = mail())}>
					<Spinner response={response_mail} />
					{$_('.action.yes')}
					<Mail class="ml-2 h-4 w-4" />
				</Button>
			{:else}
				<Button on:click={() => (response_back = return_back())}>
					<Spinner response={response_back} />
					{$_('.book.revoke')}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
