<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import SelectAccount from '$lib/components/ui/select-account/SelectAccount.svelte';
	import { Label } from '$lib/components/ui/label';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let account: string = '';
	let open = false;

	// Reset data
	$: if (open == false) {
		account = '';
	}

	let response: Promise<any>;
	async function reserve() {
		if (book) {
			book = await api.reserve(book.id, account);
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
				{$_('.book.reserve')}
			</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4">
			<div class="flex w-full flex-col gap-1.5">
				<Label for="account" class="text-left">{$_('.user.account')}</Label>
				<SelectAccount id="account" bind:account />
			</div>
		</div>
		<Dialog.Footer>
			<Button on:click={() => (response = reserve())}>
				<Spinner {response} />
				{$_('.action.apply')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
