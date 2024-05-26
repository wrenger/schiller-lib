<script lang="ts">
	import { _ } from 'svelte-i18n';
	import api from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { HandCoins, Plus } from 'lucide-svelte';
	import { Separator } from '$lib/components/ui/separator';
	import UserDialog from './UserDialog.svelte';

	export let params: api.UserSearch;
	export var onChange: (b: api.User | null) => void;

	let may_borrow: string =
		params?.may_borrow == null ? 'None' : params?.may_borrow ? 'MayBorrow' : 'MayNotBorrow';
	$: params.may_borrow = may_borrow != 'None' ? may_borrow == 'MayBorrow' : undefined;
</script>

<div class="flex gap-1">
	<DropdownMenu.Root>
		<DropdownMenu.Trigger asChild let:builder={dropdown}>
			<Tooltip.Root openDelay={0}>
				<Tooltip.Trigger asChild let:builder={tooltip}>
					<Button
						variant={!params?.may_borrow ? 'ghost' : 'outline'}
						size="icon"
						class="rounded-lg"
						aria-label={$_('.user.permission')}
						builders={[dropdown, tooltip]}
					>
						<HandCoins class="size-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" sideOffset={5}>{$_('.user.permission')}</Tooltip.Content>
			</Tooltip.Root>
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
	</UserDialog>
</div>
