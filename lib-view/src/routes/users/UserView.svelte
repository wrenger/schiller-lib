<script lang="ts">
	import { _ } from "svelte-i18n";
	import Spinner from "../../components/basic/Spinner.svelte";
	import api from "$lib/api";

	export let user: api.User | null;
	export let isNew: boolean = false;
	export var reload: (() => Promise<void>) | undefined;

	let editable: boolean = false;

	let account: string = "";
	let forename: string = "";
	let surname: string = "";
	let role: string = "";
	let may_borrow: boolean = true;

	$: if (editable || isNew || !editable || !isNew) setUser(user);
	$: if (isNew) editable = true;

	function setUser(user: api.User | null) {
		if (!isNew) {
			if (user) {
				account = user.account;
				forename = user.forename;
				surname = user.surname;
				role = user.role;
				may_borrow = user.may_borrow;
			}
		} else {
			account = "";
			forename = "";
			surname = "";
			role = "";
			may_borrow = true;
		}
	}

	let addResponse: Promise<any>;
	async function add() {
		await api.user_add({
			account,
			forename,
			surname,
			role: role ? role : "-",
			may_borrow
		});
		await onChange();
	}

	let editResponse: Promise<any>;
	async function edit() {
		await api.user_update(user?.account || "", {
			account,
			forename,
			surname,
			role: role ? role : "-",
			may_borrow
		});
		await onChange();
	}

	let deleteResponse: Promise<any>;
	async function del() {
		await api.user_delete(user?.account || "");
		await onChange();
	}

	async function onChange() {
		user = {
			account,
			forename,
			surname,
			role,
			may_borrow
		};
		if (reload) await reload();
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
			on:click={() => {
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
				id="may_borrow"
				bind:checked={may_borrow}
				disabled={!editable}
			/>
			<label class="form-check-label" for="may_borrow">{$_(".user.may-borrow")}</label>
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
		<Spinner response={addResponse} />
		{$_(".action.add")}
	</button>
	<button
		id="user-confirm-button"
		type="button"
		class="btn btn-outline-primary mt-2"
		hidden={!(editable && !isNew)}
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
		hidden={!(editable && !isNew)}
		on:click={async () => (deleteResponse = del())}
	>
		<Spinner response={deleteResponse} />
		{$_(".action.delete")}</button
	>
	<a
		id="del"
		class="btn btn-outline-primary mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew)}
		href="/books?{new URLSearchParams({ search: account })}">{$_(".user.books")}</a
	>
{/if}
