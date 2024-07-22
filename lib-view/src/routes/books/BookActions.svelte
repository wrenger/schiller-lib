<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type api from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Bookmark, BookmarkX, ClockArrowUp, Pencil, Trash2, Upload, Import } from 'lucide-svelte';
	import LendDialog from './LendDialog.svelte';
	import ReserveDialog from './ReserveDialog.svelte';
	import ReleaseDialog from './ReleaseDialog.svelte';
	import ReturnDialog from './ReturnDialog.svelte';
	import BookDialog from './BookDialog.svelte';
	import DeleteDialog from './DeleteDialog.svelte';

	export var onChange: (b: api.Book | null) => void;
	export let book: api.Book | null;
</script>

<div class="flex h-full items-center justify-between px-2">
	<div class="flex gap-1">
		<LendDialog let:dialog {book} {onChange}>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant="ghost"
						size="icon"
						class="rounded-lg"
						aria-label={book?.borrower && !book?.reservation ? $_('.book.renew') : $_('.book.lend')}
						builders={[dialog, tooltip]}
						disabled={!(book && book.borrowable)}
					>
						{#if book?.borrower}
							<ClockArrowUp class="size-5" />
						{:else}
							<Upload class="size-5" />
						{/if}
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>
					{book?.borrower && !book?.reservation ? $_('.book.renew') : $_('.book.lend')}
				</Tooltip.Content>
			</Tooltip.Root>
		</LendDialog>

		<ReturnDialog {book} {onChange} let:dialog>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant="ghost"
						size="icon"
						class="rounded-lg"
						aria-label={$_('.book.revoke')}
						builders={[dialog, tooltip]}
						disabled={!(book && book?.borrower)}
					>
						<Import class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.book.revoke')}</Tooltip.Content>
			</Tooltip.Root>
		</ReturnDialog>

		<Separator orientation="vertical" class="mx-1 mt-2 h-6" />

		{#if book && book?.reservation}
			<ReleaseDialog {book} {onChange} let:dialog>
				<Tooltip.Root openDelay={0}>
					<Tooltip.Trigger asChild let:builder={tooltip}>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg"
							aria-label={$_('.book.delete-reservation')}
							builders={[dialog, tooltip]}
							disabled={!book}
						>
							<BookmarkX class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="bottom" sideOffset={5}>
						{$_('.book.delete-reservation')}
					</Tooltip.Content>
				</Tooltip.Root>
			</ReleaseDialog>
		{:else}
			<ReserveDialog {book} {onChange} let:dialog>
				<Tooltip.Root openDelay={0}>
					<Tooltip.Trigger asChild let:builder={tooltip}>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg"
							aria-label={$_('.book.reserve')}
							builders={[dialog, tooltip]}
							disabled={!(book && book?.borrower)}
						>
							<Bookmark class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="bottom" sideOffset={5}>
						{$_('.book.reserve')}
					</Tooltip.Content>
				</Tooltip.Root>
			</ReserveDialog>
		{/if}
	</div>
	<div class="flex gap-1">
		<BookDialog {book} {onChange} let:dialog>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant="ghost"
						size="icon"
						class="rounded-lg"
						aria-label={$_('.action.edit')}
						builders={[dialog, tooltip]}
						disabled={!book}
					>
						<Pencil class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.action.edit')}</Tooltip.Content>
			</Tooltip.Root>
		</BookDialog>
		<Separator orientation="vertical" class="mx-1 mt-2 h-6" />
		<DeleteDialog {book} {onChange} let:dialog>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant="ghost"
						size="icon"
						class="rounded-lg"
						aria-label={$_('.action.delete')}
						builders={[dialog, tooltip]}
						disabled={!book}
					>
						<Trash2 class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.action.delete')}</Tooltip.Content>
			</Tooltip.Root>
		</DeleteDialog>
	</div>
</div>
