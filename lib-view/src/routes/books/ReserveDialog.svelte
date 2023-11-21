<script lang="ts">
	import { _ } from "svelte-i18n";
	import Dialog from "../../components/basic/Dialog.svelte";
	import Spinner from "../../components/basic/Spinner.svelte";
	import UserSelect from "../users/UserSelect.svelte";
	import api from "$lib/api";

	export let bookId: string;
	export let onChange: (b: api.Book | null) => void;

	export function open() {
		dialog.open();
	}

	export function close() {
		dialog.close();
	}

	let dialog: Dialog;
	let gonnaReserve: string = "";
	let reserveResponse: Promise<any>;

	async function reserve() {
		let book = await api.reserve(bookId, gonnaReserve);
		onChange(book);
		close();
	}
</script>

<Dialog bind:this={dialog}>
	<span slot="header"><h5 class="mb-0">{$_(".book.reserve")}</h5></span>
	<span slot="body">
		<UserSelect label={$_(".user")} placeholder={$_(".user.account")} bind:value={gonnaReserve} />
	</span>
	<span slot="footer">
		<button type="button" class="btn btn-primary" on:click={() => (reserveResponse = reserve())}>
			<Spinner response={reserveResponse} />
			{$_(".action.apply")}
		</button>
	</span>
</Dialog>
