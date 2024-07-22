<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let open = false;

	let response: Promise<any>;
	async function release() {
		if (book) {
			book = await api.release(book.id);
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
				{$_('.book.delete-reservation')}
			</Dialog.Title>
			<Dialog.Description>
				{$_('.book.reserved.by', { values: { '0': book?.reservation } })}
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button on:click={() => (response = release())}>
				<Spinner {response} />
				{$_('.action.apply')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
