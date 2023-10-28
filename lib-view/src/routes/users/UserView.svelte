<script lang="ts" context="module">
	export class User {
		account!: string;
		forename!: string;
		surname!: string;
		role!: string;
		permission!: boolean;
	}
</script>

<script lang="ts">
	import { _ } from "svelte-i18n";

	export let user: User | null;
	export let isNew: boolean = false;
	export var reload: (() => Promise<void>) | undefined;

	let editable: boolean = false;

	let account: string = "";
	let forename: string = "";
	let surname: string = "";
	let role: string = "";
	let permission: boolean = true;

	$: if (editable || isNew || !editable || !isNew) setUser(user);
	$: if (isNew) editable = true;

	function setUser(user: User | null) {
		if (!isNew) {
			if (user) {
				account = user.account;
				forename = user.forename;
				surname = user.surname;
				role = user.role;
				permission = user.permission;
			}
		} else {
			account = "";
			forename = "";
			surname = "";
			role = "";
			permission = true;
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		onChange();
		console.log("Add:", user);
		if (reload) await reload();
	}

	let editResponse: Promise<any>;
	async function edit() {
		onChange();
		console.log("Edit:", user);
		if (reload) await reload();
	}

	async function del() {
		console.log("Delete:", user?.account);
		user = null;
		editable = false;
		isNew = false;
		if (reload) await reload();
	}

	function onChange() {
		user = {
			account,
			forename,
			surname,
			role,
			permission
		};
		editable = false;
		isNew = false;
	}
</script>

{#if user || isNew}
	<div class="card-header d-flex justify-content-between">
		<button
			id="edit"
			class="btn btn-outline-primary {editable && !isNew ? 'active' : ''}"
			type="button"
			aria-expanded="false"
			title={$_(".action.edit")}
			disabled={!user}
			on:click={() => {
				editable = true;
				isNew = false;
			}}
		>
			<i class="bi bi-pencil-square" />
		</button>
		<button
			id="cancel"
			class="btn btn-outline-secondary"
			type="button"
			aria-expanded="false"
			title={$_(".action.close")}
			on:click={async () => {
				user = null;
				isNew = false;
				editable = false;
			}}
		>
			<i class="bi bi-x-lg" />
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
				readonly={!editable}
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
				readonly={!editable}
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
					readonly={!editable}
					bind:value={account}
				/>
				<button
					type="button"
					class="btn btn-outline-secondary"
					title={$_(".user.request")}
					disabled={!editable}
					on:click={() => console.log("Autofill")}
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
				readonly={!editable}
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
				id="permission"
				bind:checked={permission}
				disabled={!editable}
			/>
			<label class="form-check-label" for="permission">{$_(".user.may-borrow")}</label>
		</div>
	</div>
	<button
		id="user-abort-button"
		type="button"
		class="btn btn-outline-secondary mt-2"
		hidden={!editable}
		on:click={() => {
			setUser(user);
			editable = false;
			isNew = false;
		}}
	>
		{$_(".action.cancel")}
	</button>
	<button
		id="user-add-button"
		class="btn btn-outline-primary mt-2"
		type="button"
		hidden={!(editable && isNew)}
		on:click={() => (addResponse = add())}
	>
		{#await addResponse}
			<span
				id="user-add-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		{$_(".action.add")}
	</button>
	<button
		id="user-confirm-button"
		type="button"
		class="btn btn-outline-primary mt-2"
		hidden={!(editable && !isNew)}
		on:click={() => (editResponse = edit())}
	>
		{#await editResponse}
			<span
				id="user-confirm-button-spinner"
				class="spinner-border spinner-border-sm"
				role="status"
				aria-hidden="true"
			/>
		{/await}
		{$_(".action.apply")}
	</button>
	<button
		id="del"
		class="btn btn-outline-danger mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(editable && !isNew)}
		on:click={async () => {
			await del();
		}}>{$_(".action.delete")}</button
	>
	<a
		id="del"
		class="btn btn-outline-primary mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew)}
		href="/books?i={account}">{$_(".user.books")}</a
	>
{/if}
