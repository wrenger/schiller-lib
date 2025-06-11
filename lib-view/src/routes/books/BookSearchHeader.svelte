<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { categories } from '$lib/store';
	import { BookDashed, Plus, Tags } from 'lucide-svelte';
	import { Separator } from '$lib/components/ui/separator';
	import BookDialog from './BookDialog.svelte';
	import IconButton from '$lib/components/custom/IconButton.svelte';

	export let params: Omit<api.BookSearch, 'offset' | 'limit'>;
	export var onChange: (b: api.Book | null) => void;

	let category: string = params?.category == null ? 'None' : params.category;
	$: params.category = category != 'None' ? category : '';
</script>

<div class="flex gap-1">
	<DropdownMenu.Root>
		<DropdownMenu.Trigger asChild let:builder={dropdown}>
			<IconButton
				icon={BookDashed}
				label={$_('.book.state')}
				variant={params.state == 'None' ? 'ghost' : 'outline'}
				builders={[dropdown]}
				tooltip_side="bottom"
			/>
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="w-56">
			<DropdownMenu.Label>{$_('.book.state')}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<DropdownMenu.RadioGroup bind:value={params.state}>
				<DropdownMenu.RadioItem value="None">{$_('.action.all')}</DropdownMenu.RadioItem>
				<DropdownMenu.RadioItem value="Borrowable">{$_('.book.borrowable')}</DropdownMenu.RadioItem>
				<DropdownMenu.RadioItem value="NotBorrowable"
					>{$_('.book.not-borrowable')}</DropdownMenu.RadioItem
				>
				<DropdownMenu.RadioItem value="Borrowed">{$_('.book.borrowed')}</DropdownMenu.RadioItem>
				<DropdownMenu.RadioItem value="Reserved">{$_('.book.reserved')}</DropdownMenu.RadioItem>
			</DropdownMenu.RadioGroup>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
	<DropdownMenu.Root>
		<DropdownMenu.Trigger asChild let:builder={dropdown}>
			<IconButton
				icon={Tags}
				label={$_('.category')}
				variant={!params?.category ? 'ghost' : 'outline'}
				builders={[dropdown]}
				tooltip_side="bottom"
			/>
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="w-56">
			<DropdownMenu.Label>{$_('.category')}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<div class="max-h-72 overflow-y-scroll">
				<DropdownMenu.RadioGroup bind:value={category}>
					<DropdownMenu.RadioItem value="None">{$_('.action.all')}</DropdownMenu.RadioItem>
					{#each $categories as { name, id }}
						<DropdownMenu.RadioItem value={id}>{id} - {name}</DropdownMenu.RadioItem>
					{/each}
				</DropdownMenu.RadioGroup>
			</div>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
	<Separator orientation="vertical" class="mx-1 mt-2 h-6" />
	<BookDialog book={null} {onChange} let:dialog>
		<IconButton icon={Plus} label={$_('.action.add')} builders={[dialog]} tooltip_side="bottom" />
	</BookDialog>
</div>
