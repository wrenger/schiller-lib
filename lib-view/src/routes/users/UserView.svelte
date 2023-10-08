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
	export let user: User | null;
	export let isNew: boolean = false;

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
	}

	let editResponse: Promise<any>;
	async function edit() {
		onChange();
		console.log("Edit:", user);
	}
	async function del() {
		console.log("Delete:", user?.account);
		user = null;
		editable = false;
		isNew = false;
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
			title="Edit"
			disabled={!user}
			on:click={() => {
				editable = true;
				isNew = false;
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentCol ps-0or"
				class="bi bi-pencil-square"
				viewBox="0 0 16 16"
			>
				<path
					d="M15.502 1.94a.5.5 0 0 1 0 .706L14.459 3.69l-2-2L13.502.646a.5.5 0 0 1 .707 0l1.293 1.293zm-1.75 2.456-2-2L4.939 9.21a.5.5 0 0 0-.121.196l-.805 2.414a.25.25 0 0 0 .316.316l2.414-.805a.5.5 0 0 0 .196-.12l6.813-6.814z"
				/>
				<path
					fill-rule="evenodd"
					d="M1 13.5A1.5 1.5 0 0 0 2.5 15h11a1.5 1.5 0 0 0 1.5-1.5v-6a.5.5 0 0 0-1 0v6a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 .5-.5H9a.5.5 0 0 0 0-1H2.5A1.5 1.5 0 0 0 1 2.5v11z"
				/>
			</svg>
		</button>
		<button
			id="cancel"
			class="btn btn-outline-secondary"
			type="button"
			aria-expanded="false"
			title="Close"
			on:click={async () => {
				user = null;
				isNew = false;
				editable = false;
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				fill="currentCol ps-0or"
				class="bi bi-x-lg"
				viewBox="0 0 16 16"
			>
				<path
					d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"
				/>
			</svg>
		</button>
	</div>

	<div class="row pt-1 m-0">
		<div class="col ps-0">
			<label for="forename" class="form-label">Forename</label>
			<input
				id="forename"
				type="text"
				class="form-control"
				placeholder="Forename"
				aria-label="Forename"
				readonly={!editable}
				bind:value={forename}
			/>
		</div>
		<div class="col ps-0">
			<label for="surname" class="form-label">Surname</label>
			<input
				id="surname"
				type="text"
				class="form-control"
				placeholder="Surname"
				aria-label="Surname"
				readonly={!editable}
				bind:value={surname}
			/>
		</div>
	</div>
	<div class="row m-0">
		<div class="col ps-0">
			<label for="account" class="form-label">Account</label>
			<div class="input-group" id="account">
				<input
					type="text"
					class="form-control"
					placeholder="Account"
					aria-label="Account"
					readonly={!editable}
					bind:value={account}
				/>
				<button
					type="button"
					class="btn btn-outline-secondary"
					title="Autofill"
					disabled={!editable}
					on:click={() => console.log("Autofill")}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						fill="currentColor"
						class="bi bi-upload"
						viewBox="0 0 16 16"
					>
						<path
							d="M.5 9.9a.5.5 0 0 1 .5.5v2.5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2.5a.5.5 0 0 1 1 0v2.5a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-2.5a.5.5 0 0 1 .5-.5z"
						/>
						<path
							d="M7.646 1.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1-.708.708L8.5 2.707V11.5a.5.5 0 0 1-1 0V2.707L5.354 4.854a.5.5 0 1 1-.708-.708l3-3z"
						/>
					</svg>
				</button>
			</div>
		</div>
		<div class="col ps-0">
			<label for="role" class="form-label">Group</label>
			<input
				id="role"
				type="text"
				class="form-control"
				placeholder="Group"
				aria-label="Group"
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
			<label class="form-check-label" for="permission">Can Borrow</label>
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
		Cancel
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
		Add
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
		Confirm
	</button>
	<button
		id="del"
		class="btn btn-outline-danger mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(editable && !isNew)}
		on:click={async () => {
			await del();
		}}>Delete</button
	>
	<a
		id="del"
		class="btn btn-outline-primary mt-2"
		type="button"
		aria-expanded="false"
		hidden={!(!editable && !isNew)}
		href="/books?i={account}">Borrowed/Reserved Books</a
	>
{/if}
