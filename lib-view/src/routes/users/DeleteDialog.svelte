<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	export let user: api.User | null;
	export var onChange: (u: api.User | null) => void;

	let open = false;

	let response: Promise<any>;
	async function del() {
		if (user) {
			await api.user_delete(user.account);
			user = null;
			open = false;
			onChange(user);
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
				{$_('.user.delete')}
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button on:click={() => (response = del())}>
				<Spinner {response} />
				{$_('.action.ok')}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
