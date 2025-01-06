<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import { Separator } from '$lib/components/ui/separator';
	import { BookCopy, Pencil, Trash2 } from 'lucide-svelte';
	import UserDialog from './UserDialog.svelte';
	import DeleteDialog from '../../lib/components/custom/DeleteDialog.svelte';
	import { goto } from '$app/navigation';
	import { handle_result } from '$lib';
	import IconButton from '$lib/components/custom/IconButton.svelte';

	export var onChange: (u: api.User | null) => void;
	export let user: api.User | null;

	async function onDelete() {
		handle_result(await api.user_delete(user?.account || ''));
		user = null;
		onChange(null);
	}
</script>

<div class="flex h-full items-center justify-between px-2">
	<div class="flex gap-1">
		<IconButton
			icon={BookCopy}
			label={$_('.user.books')}
			disabled={!user}
			onClick={() => goto(`/books?${new URLSearchParams({ search: user?.account || '' })}`)}
		/>
	</div>
	<div class="flex gap-1">
		<UserDialog {user} {onChange} let:dialog>
			<IconButton icon={Pencil} label={$_('.action.edit')} builders={[dialog]} disabled={!user} />
		</UserDialog>
		<Separator orientation="vertical" class="mx-1 mt-2 h-6" />
		<DeleteDialog identifier={user?.account || ''} {onDelete} let:dialog>
			<IconButton
				icon={Trash2}
				label={$_('.action.delete')}
				builders={[dialog]}
				disabled={!user}
				class="text-destructive hover:text-destructive"
			/>
		</DeleteDialog>
	</div>
</div>
