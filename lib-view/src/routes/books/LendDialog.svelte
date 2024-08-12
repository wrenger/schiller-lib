<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { handle_result, onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import SelectAccount from '$lib/components/ui/select-account/SelectAccount.svelte';
	import { Label } from '$lib/components/ui/label';
	import DateInput from '$lib/components/ui/date-input/DateInput.svelte';
	import { DateTime } from 'luxon';
	import { settingsGlobal } from '$lib/store';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let account: string = book?.reservation ?? book?.borrower?.user ?? '';
	let period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
	let open = false;

	// Reset data
	$: if (open == false) {
		account = book?.reservation ? book.reservation : book?.borrower ? book.borrower.user : '';
		period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
	}

	let response: Promise<any>;
	async function lend() {
		if (book) {
			book = handle_result(
				await api.lending_lend({ id: book.id, account, deadline: period?.toISODate() })
			);
			open = false;
			onChange(book);
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
				{book?.borrower && !book?.reservation ? $_('.book.renew') : $_('.book.lend')}
			</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4">
			<div class="flex w-full flex-col gap-1.5">
				<Label for="account" class="text-left">{$_('.user.account')}</Label>
				{#if book?.reservation || book?.borrower}
					<!-- Read-only for renews and reservations! -->
					<div>{account}</div>
				{:else}
					<SelectAccount id="account" bind:account />
				{/if}
			</div>
			<DateInput bind:date={period} label={$_('.book.lend.period')} />
		</div>
		<Dialog.Footer>
			<Button on:click={() => (response = lend())}>
				<Spinner {response} />
				{$_('.action.apply')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
