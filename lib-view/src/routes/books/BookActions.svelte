<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import { Separator } from '$lib/components/ui/separator';
	import { Bookmark, BookmarkX, ClockArrowUp, Pencil, Trash2, Upload, Import } from 'lucide-svelte';
	import LendDialog from './LendDialog.svelte';
	import ReserveDialog from './ReserveDialog.svelte';
	import ReleaseDialog from './ReleaseDialog.svelte';
	import ReturnDialog from './ReturnDialog.svelte';
	import BookDialog from './BookDialog.svelte';
	import DeleteDialog from '$lib/components/custom/DeleteDialog.svelte';
	import IconButton from '$lib/components/custom/IconButton.svelte';

	export var onChange: (b: api.Book | null) => void;
	export let book: api.Book | null;

	async function onDelete() {
		await api.book_delete(book?.id || '');
		book = null;
		onChange(null);
	}
</script>

<div class="flex h-full items-center justify-between px-2">
	<div class="flex gap-1">
		<LendDialog let:dialog {book} {onChange}>
			<IconButton
				icon={book?.borrower && !book?.reservation ? ClockArrowUp : Upload}
				label={book?.borrower && !book?.reservation ? $_('.book.renew') : $_('.book.lend')}
				builders={[dialog]}
				disabled={!book || !book.borrowable || !!book.borrower}
			/>
		</LendDialog>

		<ReturnDialog {book} {onChange} let:dialog>
			<IconButton
				icon={Import}
				label={$_('.book.revoke')}
				builders={[dialog]}
				disabled={!(book && book?.borrower)}
			/>
		</ReturnDialog>

		<Separator orientation="vertical" class="mx-1 mt-2 h-6" />

		{#if book && book?.reservation}
			<ReleaseDialog {book} {onChange} let:dialog>
				<IconButton
					icon={BookmarkX}
					label={$_('.book.delete-reservation')}
					builders={[dialog]}
					disabled={!book}
					tooltip_side="bottom"
				/>
			</ReleaseDialog>
		{:else}
			<ReserveDialog {book} {onChange} let:dialog>
				<IconButton
					icon={Bookmark}
					label={$_('.book.reserve')}
					builders={[dialog]}
					disabled={!(book && book?.borrower)}
					tooltip_side="bottom"
				/>
			</ReserveDialog>
		{/if}
	</div>
	<div class="flex gap-1">
		<BookDialog {book} {onChange} let:dialog>
			<IconButton
				icon={Pencil}
				label={$_('.action.edit')}
				builders={[dialog]}
				disabled={!book}
				tooltip_side="bottom"
			/>
		</BookDialog>
		<Separator orientation="vertical" class="mx-1 mt-2 h-6" />
		<DeleteDialog identifier={book?.title || ''} {onDelete} let:dialog>
			<IconButton
				icon={Trash2}
				label={$_('.action.delete')}
				builders={[dialog]}
				disabled={!book}
				tooltip_side="bottom"
				class="text-destructive hover:text-destructive"
			/>
		</DeleteDialog>
	</div>
</div>
