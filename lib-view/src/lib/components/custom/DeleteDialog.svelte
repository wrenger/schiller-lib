<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	export let identifier: string;
	export let onDelete: () => Promise<void>;
	let response: Promise<any>;
	let open = false;

	function del() {
		response = onDelete();
		open = false;
	}
</script>

<Dialog.Root bind:open {onOutsideClick}>
	<Dialog.Trigger asChild let:builder={dialog}>
		<slot {dialog} />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{$_('.action.delete')}</Dialog.Title>
			<Dialog.Description>
				{$_('.user.delete', { values: { '0': identifier } })}
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button variant="destructive" on:click={del}>
				<Spinner {response} />
				{$_('.action.delete')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
