<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Spinner from '../../components/basic/Spinner.svelte';
	import api from '$lib/api';

	export var onChange: ((b: api.User | null) => void) | undefined;
	export function display(u: api.User) {
		state = {
			kind: State.Display,
			user: u,
			editing: false
		};
	}
	export function create() {
		state = { kind: State.Create };
	}

	enum State {
		Display,
		Create
	}
	interface Display {
		kind: State.Display;
		user: api.User;
		editing: boolean;
	}
	interface Create {
		kind: State.Create;
	}
	let state: Display | Create = { kind: State.Create };

	let account: string = '';
	let forename: string = '';
	let surname: string = '';
	let role: string = '';
	let may_borrow: boolean = true;

	let userInfoResponse: Promise<any>;

	$: if (state.kind == State.Display) setUser(state.user);
	$: if (state.kind == State.Create) setUser(null);

	function setUser(user: api.User | null) {
		if (user != null) {
			account = user.account;
			forename = user.forename;
			surname = user.surname;
			role = user.role;
			may_borrow = user.may_borrow;
		} else {
			account = '';
			forename = '';
			surname = '';
			role = '';
			may_borrow = true;
		}
	}

	function getUser(): api.User {
		return {
			account,
			forename,
			surname,
			role: role ? role : '-',
			may_borrow
		};
	}

	let addResponse: Promise<void>;
	async function add() {
		if (state.kind == State.Create) {
			let user = getUser();
			await api.user_add(user);
			onChangeInner(user);
		}
	}

	let editResponse: Promise<void>;
	async function edit() {
		if (state.kind == State.Display) {
			let user = getUser();
			await api.user_update(user?.account ?? '', user);
			onChangeInner(user);
		}
	}

	let delResponse: Promise<void>;
	async function del() {
		if (state.kind == State.Display) {
			await api.user_delete(state.user.account);
			onChangeInner(null);
		}
	}

	function onChangeInner(user: api.User | null) {
		if (user != null && state.kind === State.Display) {
			state.user = user;
			state.editing = false;
			user = {
				account,
				forename,
				surname,
				role,
				may_borrow
			};
		}
		if (onChange) onChange(user);
	}
</script>

<div class="w-full max-h-full text-token card p-2 space-y-2 overflow-y-scroll overflow-x-hidden">
	<div class="flex p-2 pb-0">
		<span class="flex-auto">
			<button
				id="cancel"
				class="btn-icon variant-filled"
				type="button"
				aria-expanded="false"
				title={$_('.action.close')}
				on:click={() => onChangeInner(null)}
			>
				<i class="fa-solid fa-angle-left"></i>
			</button>
		</span>
		<span>
			<button
				id="edit"
				class="btn-icon variant-filled{state.kind === State.Display && state.editing
					? '-primary'
					: ''}"
				type="button"
				aria-expanded="false"
				title={$_('.action.edit')}
				disabled={state.kind === State.Create}
				on:click={() => {
					if (state.kind === State.Display) state.editing = true;
				}}
			>
				<i class="fa-solid fa-pen-to-square"></i>
			</button>
		</span>
	</div>
	<div class="w-full grid grid-cols-2 gap-4">
		<label class="label">
			<span>{$_('.user.forename')}</span>
			<input
				class="input"
				type="text"
				placeholder={$_('.user.forename')}
				readonly={!(state.kind === State.Create || state.editing)}
				bind:value={forename}
			/>
		</label>

		<label class="label">
			<span>{$_('.user.surname')}</span>
			<input
				class="input"
				type="text"
				placeholder={$_('.user.surname')}
				readonly={!(state.kind === State.Create || state.editing)}
				bind:value={surname}
			/>
		</label>

		<label class="label">
			<span>{$_('.user.account')}</span>
			<div class="input-group grid-cols-[1fr_auto] mb-2">
				<input
					class="input"
					type="text"
					placeholder={$_('.user.account')}
					readonly={!(state.kind === State.Create || state.editing)}
					bind:value={account}
				/>
				<button
					class="variant-soft"
					type="button"
					title={$_('.user.request')}
					disabled={!(state.kind === State.Create || state.editing)}
					on:click={async () => {
						userInfoResponse = api.user_fetch(account);
						let data = await userInfoResponse;
						forename = data.forename;
						surname = data.surname;
						account = data.account;
						role = data.role;
					}}
				>
					<Spinner response={userInfoResponse} />
					<i class="fa-solid fa-download"></i>
				</button>
			</div>
		</label>

		<label class="label">
			<span>{$_('.user.role')}</span>
			<input
				class="input"
				type="text"
				placeholder={$_('.user.role')}
				readonly={!(state.kind === State.Create || state.editing)}
				bind:value={role}
			/>
		</label>
		<label class="flex items-center space-x-2">
			<input
				class="checkbox"
				type="checkbox"
				bind:checked={may_borrow}
				disabled={!(state.kind === State.Create || state.editing)}
			/>
			<p>{$_('.user.may-borrow')}</p>
		</label>
	</div>

	<div class="p-2 pt-0 flex space-x-2 justify-center">
		{#if state.kind === State.Create || (state.kind === State.Display && state.editing)}
			<button
				id="user-abort-button"
				type="button"
				class="btn variant-filled mt-2"
				on:click={() => {
					if (state.kind === State.Display) {
						state.editing = false;
						setUser(state.user);
					} else {
						onChangeInner(null);
					}
				}}
			>
				{$_('.action.cancel')}
			</button>
		{/if}

		{#if state.kind === State.Create}
			<button
				id="user-add-button"
				class="btn variant-filled-primary mt-2"
				type="button"
				on:click={() => (addResponse = add())}
			>
				<Spinner response={addResponse} />
				{$_('.action.add')}
			</button>
		{:else if state.kind === State.Display && state.editing}
			<button
				id="user-confirm-button"
				type="button"
				class="btn variant-filled-primary mt-2"
				on:click={() => (editResponse = edit())}
			>
				<Spinner response={editResponse} />
				{$_('.action.apply')}
			</button>
			<button
				id="del"
				class="btn variant-filled-error mt-2"
				type="button"
				aria-expanded="false"
				on:click={async () => (delResponse = del())}
			>
				<Spinner response={delResponse} />
				{$_('.action.delete')}</button
			>
		{:else}
			<a
				id="del"
				class="btn variant-filled-primary mt-2"
				type="button"
				aria-expanded="false"
				href="/books?{new URLSearchParams({ search: account })}"
			>
				{$_('.user.books')}
			</a>
		{/if}
	</div>
</div>
