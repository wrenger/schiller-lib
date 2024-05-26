<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { categories } from '$lib/store';
	import { BookDashed, Plus, Tags } from 'lucide-svelte';
	import { Separator } from '$lib/components/ui/separator';
	import BookDialog from './BookDialog.svelte';

	export let params: api.BookSearch;
	export var onChange: (b: api.Book | null) => void;

	let category: string = params?.category == null ? 'None' : params.category;
	$: params.category = category != 'None' ? category : undefined;
</script>

<div class="flex gap-1">
	<DropdownMenu.Root>
		<DropdownMenu.Trigger asChild let:builder={dropdown}>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant={params.state == 'None' ? 'ghost' : 'outline'}
						size="icon"
						class="rounded-lg"
						aria-label={$_('.book.state')}
						builders={[dropdown, tooltip]}
					>
						<BookDashed class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.book.state')}</Tooltip.Content>
			</Tooltip.Root>
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
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant={!params?.category ? 'ghost' : 'outline'}
						size="icon"
						class="rounded-lg"
						aria-label={$_('.category')}
						builders={[dropdown, tooltip]}
					>
						<Tags class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.category')}</Tooltip.Content>
			</Tooltip.Root>
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
		<Tooltip.Root openDelay={0}>
			<Tooltip.Trigger asChild let:builder={tooltip}>
				<Button
					variant="ghost"
					size="icon"
					class="rounded-lg"
					aria-label={$_('.action.add')}
					builders={[dialog, tooltip]}
				>
					<Plus class="size-5" />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content side="bottom" sideOffset={5}>{$_('.action.add')}</Tooltip.Content>
		</Tooltip.Root>
	</BookDialog>
</div>
