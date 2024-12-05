<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { handle_result, onOutsideClick } from '$lib';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import api from '$lib/api';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { Label } from '$lib/components/ui/label';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Download } from 'lucide-svelte';

	export let user: api.User | null;
	export var onChange: (u: api.User | null) => void;

	let forename = '';
	let surname = '';
	let account = '';
	let role = '';
	let may_borrow = true;

	function setUser() {
		if (user) {
			forename = user.forename;
			surname = user.surname;
			account = user.account;
			if (user.role) role = user.role;
			if (user.may_borrow) may_borrow = user.may_borrow;
		} else {
			forename = '';
			surname = '';
			account = '';
			role = '';
			may_borrow = true;
		}
	}

	function getUser(): api.User {
		return {
			forename,
			surname,
			account,
			role,
			may_borrow
		};
	}

	let open = false;
	let userInfoResponse: Promise<any>;

	$: if (open) setUser();

	let addResponse: Promise<any>;
	async function add() {
		let user = handle_result(await api.user_add(getUser()));
		open = false;
		onChange(user);
	}

	let editResponse: Promise<any>;
	async function edit() {
		if (user) {
			let newUser = handle_result(await api.user_update(user.account, getUser()));
			open = false;
			onChange(newUser);
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={(value) => (open = value)} {onOutsideClick}>
	<Dialog.Trigger asChild let:builder={dialog}>
		<slot {dialog} />
	</Dialog.Trigger>
	<Dialog.Content class="max-h-full overflow-y-scroll">
		<Dialog.Header>
			<Dialog.Title>
				{#if user}
					{$_('.action.edit')}
				{:else}
					{$_('.action.add')}
				{/if}
			</Dialog.Title>
		</Dialog.Header>
		<div class="grid gap-4">
			<div class="grid grid-cols-2 space-x-1">
				<div class="flex w-full flex-col gap-1.5">
					<Label for="forename" class="text-left">{$_('.user.forename')}</Label>
					<Input id="forename" placeholder={$_('.user.forename')} bind:value={forename} />
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="surname" class="text-left">{$_('.user.surname')}</Label>
					<Input id="surname" placeholder={$_('.user.surname')} bind:value={surname} />
				</div>
			</div>
			<div class="grid grid-cols-2 space-x-1">
				<div class="flex w-full flex-col gap-1.5">
					<Label for="isbn" class="text-left">{$_('.user.account')}</Label>
					<div class="relative">
						<Button
							size="icon"
							variant="ghost"
							title={$_('.user.request')}
							class="text-muted-foreground absolute left-2 top-2.5 h-5 w-5 p-[2px]"
							on:click={async () => {
								userInfoResponse = api.user_fetch_data(account);
								let data = handle_result(await userInfoResponse);
								forename = data.forename;
								surname = data.surname;
								account = data.account;
								role = data.role;
							}}
						>
							<Spinner response={userInfoResponse} spinnerClass="size-5 !mr-0">
								<Download class="size-5" />
							</Spinner>
						</Button>
						<Input id="isbn" class="pl-8" placeholder={$_('.user.account')} bind:value={account} />
					</div>
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="role" class="text-left">{$_('.user.role')}</Label>
					<Input id="role" placeholder={$_('.user.role')} bind:value={role} />
				</div>
			</div>
			<div class="flex items-center space-x-2">
				<Checkbox
					id="may-borrowable"
					bind:checked={may_borrow}
					aria-labelledby="may-borrowable-label"
				/>
				<Label
					id="may-borrowable-label"
					for="may-borrowable"
					class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
				>
					{$_('.user.may-borrow')}
				</Label>
			</div>
		</div>
		<Dialog.Footer>
			{#if user}
				<Button on:click={() => (editResponse = edit())}>
					<Spinner response={editResponse} />
					{$_('.action.apply')}
				</Button>
			{:else}
				<Button on:click={() => (addResponse = add())}>
					<Spinner response={addResponse} />
					{$_('.action.apply')}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
