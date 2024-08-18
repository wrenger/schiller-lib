<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { handle_result, onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	export let book: api.Book | null;
	export var onChange: (b: api.Book | null) => void;

	let open = false;

	let response: Promise<any>;
	async function del() {
		if (book) {
			handle_result(await api.book_delete(book.id));
			book = null;
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
				{$_('.action.delete')}
			</Dialog.Title>
			<Dialog.Description>
				{$_('.book.delete')}
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button on:click={() => (response = del())} variant="destructive">
				<Spinner {response} />
				{$_('.action.delete')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
