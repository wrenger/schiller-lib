<script lang="ts">
	import { _ } from "svelte-i18n";
	import Spinner from "../../components/basic/Spinner.svelte";
	import api from "$lib/api";

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

	let account: string = "";
	let forename: string = "";
	let surname: string = "";
	let role: string = "";
	let may_borrow: boolean = true;

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
			account = "";
			forename = "";
			surname = "";
			role = "";
			may_borrow = true;
		}
	}

	function getUser(): api.User {
		return {
			account,
			forename,
			surname,
			role: role ? role : "-",
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
			await api.user_update(user?.account ?? "", user);
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

<div class="card-header d-flex justify-content-between">
	<button
		id="cancel"
		class="btn btn-outline-secondary"
		type="button"
		aria-expanded="false"
		title={$_(".action.close")}
		on:click={() => onChangeInner(null)}
	>
		<i class="bi bi-caret-left-fill" />
	</button>
	<button
		id="edit"
		class="btn btn-outline-primary"
		class:active={state.kind === State.Display && state.editing}
		type="button"
		aria-expanded="false"
		title={$_(".action.edit")}
		disabled={state.kind === State.Create}
		on:click={() => {
			if (state.kind === State.Display) state.editing = true;
		}}
	>
		<i class="bi bi-pencil-square" />
	</button>
</div>

<div class="row pt-1 m-0">
	<div class="col ps-0">
		<label for="forename" class="form-label">{$_(".user.forename")}</label>
		<input
			id="forename"
			type="text"
			class="form-control"
			placeholder={$_(".user.forename")}
			aria-label={$_(".user.forename")}
			readonly={!(state.kind === State.Create || state.editing)}
			bind:value={forename}
		/>
	</div>
	<div class="col ps-0 pe-0">
		<label for="surname" class="form-label">{$_(".user.surname")}</label>
		<input
			id="surname"
			type="text"
			class="form-control"
			placeholder={$_(".user.surname")}
			aria-label={$_(".user.surname")}
			readonly={!(state.kind === State.Create || state.editing)}
			bind:value={surname}
		/>
	</div>
</div>
<div class="row m-0">
	<div class="col ps-0">
		<label for="account" class="form-label">{$_(".user.account")}</label>
		<div class="input-group" id="account">
			<input
				type="text"
				class="form-control"
				placeholder={$_(".user.account")}
				aria-label={$_(".user.account")}
				readonly={!(state.kind === State.Create || state.editing)}
				bind:value={account}
			/>
			<button
				type="button"
				class="btn btn-outline-secondary"
				title={$_(".user.request")}
				disabled={!(state.kind === State.Create || state.editing)}
				on:click={async () => {
					let data = await api.user_fetch(account);
					forename = data.forename;
					surname = data.surname;
					account = data.account;
					role = data.role;
				}}
			>
				<i class="bi bi-upload" />
			</button>
		</div>
	</div>
	<div class="col ps-0 pe-0">
		<label for="role" class="form-label">{$_(".user.role")}</label>
		<input
			id="role"
			type="text"
			class="form-control"
			placeholder={$_(".user.role")}
			aria-label={$_(".user.role")}
			readonly={!(state.kind === State.Create || state.editing)}
			bind:value={role}
		/>
	</div>
</div>
<div class="row m-0 pt-1">
	<div class="form-check">
		<input
			class="form-check-input"
			type="checkbox"
			value=""
			id="may_borrow"
			bind:checked={may_borrow}
			disabled={!(state.kind === State.Create || state.editing)}
		/>
		<label class="form-check-label" for="may_borrow">{$_(".user.may-borrow")}</label>
	</div>
</div>

<button
	id="user-abort-button"
	type="button"
	class="btn btn-outline-secondary mt-2"
	hidden={!(state.kind === State.Create || state.editing)}
	on:click={() => {
		if (state.kind === State.Display) {
			state.editing = false;
			setUser(state.user);
		} else {
			onChangeInner(null);
		}
	}}
>
	{$_(".action.cancel")}
</button>

<button
	id="user-add-button"
	class="btn btn-outline-primary mt-2"
	type="button"
	hidden={state.kind !== State.Create}
	on:click={() => (addResponse = add())}
>
	<Spinner response={addResponse} />
	{$_(".action.add")}
</button>

<button
	id="user-confirm-button"
	type="button"
	class="btn btn-outline-primary mt-2"
	hidden={!(state.kind === State.Display && state.editing)}
	on:click={() => (editResponse = edit())}
>
	<Spinner response={editResponse} />
	{$_(".action.apply")}
</button>
<button
	id="del"
	class="btn btn-outline-danger mt-2"
	type="button"
	aria-expanded="false"
	hidden={!(state.kind === State.Display && state.editing)}
	on:click={async () => (delResponse = del())}
>
	<Spinner response={delResponse} />
	{$_(".action.delete")}</button
>
<a
	id="del"
	class="btn btn-outline-primary mt-2"
	type="button"
	aria-expanded="false"
	hidden={!(state.kind === State.Display && !state.editing)}
	href="/books?{new URLSearchParams({ search: account })}"
>
	{$_(".user.books")}
</a>
