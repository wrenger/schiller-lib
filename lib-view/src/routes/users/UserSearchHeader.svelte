<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { BookLock, Plus } from 'lucide-svelte';
	import { Separator } from '$lib/components/ui/separator';
	import UserDialog from './UserDialog.svelte';
	import IconButton from '$lib/components/custom/IconButton.svelte';

	export let params: Omit<api.UserSearch, 'offset' | 'limit'>;
	export var onChange: (b: api.User | null) => void;

	const to_str = (mb: boolean | null): string =>
		mb == null ? 'None' : mb ? 'MayBorrow' : 'MayNotBorrow';
	const from_str = (mb: string): boolean | null => (mb == 'None' ? null : mb == 'MayBorrow');

	let may_borrow: string = to_str(params.may_borrow);
	$: params.may_borrow = from_str(may_borrow);
</script>

<div class="flex gap-1">
	<DropdownMenu.Root>
		<DropdownMenu.Trigger asChild let:builder={dropdown}>
			<IconButton
				icon={BookLock}
				label={$_('.user.permission')}
				variant={may_borrow === 'None' ? 'ghost' : 'outline'}
				builders={[dropdown]}
				tooltip_side="bottom"
			/>
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="w-56">
			<DropdownMenu.Label>{$_('.user.permission')}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<DropdownMenu.RadioGroup bind:value={may_borrow}>
				<DropdownMenu.RadioItem value="None">{$_('.action.all')}</DropdownMenu.RadioItem>
				<DropdownMenu.RadioItem value="MayBorrow">{$_('.user.may-borrow')}</DropdownMenu.RadioItem>
				<DropdownMenu.RadioItem value="MayNotBorrow"
					>{$_('.user.may-not-borrow')}</DropdownMenu.RadioItem
				>
			</DropdownMenu.RadioGroup>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
	<Separator orientation="vertical" class="mx-1 mt-2 h-6" />
	<UserDialog user={null} {onChange} let:dialog>
		<IconButton icon={Plus} label={$_('.action.add')} builders={[dialog]} tooltip_side="bottom" />
	</UserDialog>
</div>
