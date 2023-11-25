<script lang="ts">
	import { _ } from "svelte-i18n";
	import { DateTime } from "luxon";
	import Dialog from "../../components/basic/Dialog.svelte";
	import { settingsGlobal } from "$lib/store";
	import UserSelect from "../users/UserSelect.svelte";
	import DateField from "../../components/basic/DateField.svelte";
	import Spinner from "../../components/basic/Spinner.svelte";
	import api from "$lib/api";

	export let bookId: string;
	export let onChange: (b: api.Book | null) => void;

	export function open(account: string | undefined = undefined) {
		borrower = account ?? "";
		period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
		dialog.open();
	}

	export function close() {
		dialog.close();
	}

	let dialog: Dialog;
	let period = DateTime.local().plus({ days: $settingsGlobal.borrowing_duration });
	let borrower: string = "";
	let response: Promise<void>;

	async function lend() {
		let book = await api.lend(bookId, borrower, period?.toISODate() ?? "");
		onChange(book);
		close();
	}
</script>

<Dialog bind:this={dialog}>
	<span slot="header"><h5 class="mb-0">{$_(".book.lend")}</h5></span>
	<span slot="body">
		<UserSelect label={$_(".user")} placeholder={$_(".user.account")} bind:value={borrower} />
		<label for="period" class="sform-label">{$_(".book.lend.period")}</label>
		<DateField bind:date={period} id="period" />
	</span>
	<span slot="footer">
		<button
			id="book-confirm-button"
			type="button"
			class="btn btn-primary"
			on:click={() => (response = lend())}
		>
			<Spinner {response} />
			{$_(".action.apply")}
		</button>
	</span>
</Dialog>
