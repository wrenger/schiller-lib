<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type api from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { BookCopy, Pencil, Trash2 } from 'lucide-svelte';
	import UserDialog from './UserDialog.svelte';
	import DeleteDialog from './DeleteDialog.svelte';
	import { goto } from '$app/navigation';

	export var onChange: (u: api.User | null) => void;
	export let user: api.User | null;
</script>

<div class="flex h-full items-center justify-between px-2">
	<div class="flex gap-1">
		<Tooltip.Root openDelay={0}>
			<Tooltip.Trigger asChild let:builder={tooltip}>
				<Button
					variant="ghost"
					size="icon"
					class="rounded-lg"
					aria-label={$_('.user.books')}
					builders={[tooltip]}
					disabled={!user}
					on:click={() => goto(`/books?${new URLSearchParams({ search: user?.account || '' })}`)}
				>
					<BookCopy class="size-5" />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content side="bottom" sideOffset={5}>
				{$_('.user.books')}
			</Tooltip.Content>
		</Tooltip.Root>
	</div>
	<div class="flex gap-1">
		<UserDialog {user} {onChange} let:dialog>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant="ghost"
						size="icon"
						class="rounded-lg"
						aria-label={$_('.action.edit')}
						builders={[dialog, tooltip]}
						disabled={!user}
					>
						<Pencil class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.action.edit')}</Tooltip.Content>
			</Tooltip.Root>
		</UserDialog>
		<Separator orientation="vertical" class="mx-1 mt-2 h-6" />
		<DeleteDialog {user} {onChange} let:dialog>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant="ghost"
						size="icon"
						class="rounded-lg"
						aria-label={$_('.action.delete')}
						builders={[dialog, tooltip]}
						disabled={!user}
					>
						<Trash2 class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.action.delete')}</Tooltip.Content>
			</Tooltip.Root>
		</DeleteDialog>
	</div>
</div>
