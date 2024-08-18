<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils.js';
	import { onMount, tick } from 'svelte';
	import api from '$lib/api';
	import { _ } from 'svelte-i18n';
	import { handle_result } from '$lib';

	export let account: string;
	export let id: string = '';

	const LIMIT = 10;
	let input = '';
	let items: api.Limited<api.User>;
	let mounted: boolean = false;
	let open = false;

	onMount(() => (mounted = true));

	function closeAndFocusTrigger(triggerId: string) {
		open = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}

	async function fetch(value: string) {
		items = handle_result(
			await api.user_search({ query: value, may_borrow: null, offset: 0, limit: LIMIT })
		);
		items.rows.sort();
	}

	$: if (mounted) fetch(input);
</script>

<Popover.Root bind:open let:ids>
	<Popover.Trigger asChild let:builder>
		<Button
			{id}
			builders={[builder]}
			variant="outline"
			role="combobox"
			aria-expanded={open}
			class="w-full justify-between"
		>
			{#if account}
				{account}
			{:else}
				{$_('.action.select')}
			{/if}
			<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-fit p-0" align="start">
		<Command.Root>
			<Command.Input placeholder={$_('.search.action')} bind:value={input} />
			<div class="!max-h-48 overflow-x-hidden overflow-y-scroll">
				<Command.Empty class="h-fit">{$_('.error.none')}</Command.Empty>
				<Command.Group>
					{#each items?.rows as item (item.account)}
						<Command.Item
							value={item.account}
							onSelect={(currentValue) => {
								account = currentValue;
								closeAndFocusTrigger(ids.trigger);
							}}
						>
							<Check class={cn('mr-2 h-4 w-4', account !== item.account && 'text-transparent')} />
							{item.forename}
							{item.surname}
						</Command.Item>
					{/each}
				</Command.Group>
			</div>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
