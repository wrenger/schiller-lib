<script lang="ts">
	import { _ } from "svelte-i18n";
	import Dialog from "./Dialog.svelte";

	let dialog: Dialog;
	let err: string;

	export async function request(
		url: string,
		type: string,
		json: BodyInit | null | undefined
	): Promise<any> {
		try {
			const response = await fetch(url, {
				method: type,
				headers: {
					"Content-Type": "application/json; charset=utf-8"
				},
				body: json
			});

			if (response.status === 200) {
				const contentType = response.headers.get("Content-Type");

				if (contentType && contentType.includes("application/json")) {
					return response.json();
				} else {
					return response.text();
				}
			} else {
				const data = await response.text();
				throw data;
			}
		} catch (error) {
			if (dialog) dialog.open();
			err = error_msg(error as string);
			throw error;
		}
	}

	enum ServerError {
		Arguments,
		Logic,
		FileNotFound,
		FileOpen,
		SQL,
		Network,
		InvalidFormat,
		NothingFound,
		// Specific errors
		InvalidBook,
		InvalidUser,
		// Lending errors
		LendingUserMayNotBorrow,
		LendingBookNotBorrowable,
		LendingBookAlreadyBorrowed,
		LendingBookAlreadyBorrowedByUser,
		LendingBookNotBorrowed,
		LendingBookAlreadyReserved,
		// Migration
		UnsupportedProjectVersion
	}

	function error_msg(code: string): string {
		switch (parseInt(code)) {
			case ServerError.Arguments:
				return "error.input";
			case ServerError.Logic:
				return "error.update";
			case ServerError.FileNotFound:
				return ".error.file-open";
			case ServerError.FileOpen:
				return ".error.file-open";
			case ServerError.SQL:
				return ".error.sql";
			case ServerError.Network:
				return ".error.network";
			case ServerError.InvalidFormat:
				return ".error.format";
			case ServerError.NothingFound:
				return ".error.none";
			case ServerError.InvalidBook:
				return ".book.invalid";
			case ServerError.InvalidUser:
				return ".user.invalid";
			case ServerError.LendingUserMayNotBorrow:
				return ".error.lending.user";
			case ServerError.LendingBookNotBorrowable:
				return ".error.lending.book";
			case ServerError.LendingBookAlreadyBorrowed:
				return ".error.lending.already-borrowed";
			case ServerError.LendingBookAlreadyBorrowedByUser:
				return ".error.lending.already-borrowed-by";
			case ServerError.LendingBookNotBorrowed:
				return ".error.lending.not-borrowed";
			case ServerError.LendingBookAlreadyReserved:
				return ".error.lending.already-reserved";
			case ServerError.UnsupportedProjectVersion:
				return ".error.update";
			default:
				return ".error.unknown";
		}
	}
</script>

<Dialog bind:this={dialog} min={"fit"} size={"small"}>
	<h5 slot="header" class="m-0">{$_(".alert.error")}</h5>
	<span slot="body">
		<p class="m-0 fs-6">{$_(err)}</p>
	</span>
	<span slot="footer">
		{#if err == ".error.unknown"}
			<a class="btn btn-danger" href="auth/logout">{$_(".action.logout")}</a>
		{/if}
	</span>
</Dialog>
