<script lang="ts">
	import { _ } from "svelte-i18n";
	import Dialog from "./Dialog.svelte";

	let dialog: Dialog;
	let err: string;

	/** Safely create a valid query string from the provided query parameters */
	function query_str(params: Record<string, any>): string {
		if (params) {
			let data: Record<string, string> = {};
			for (let key in params) {
				if (params[key] != undefined && params[key] != null) data[key] = params[key].toString();
			}
			// the URLSearchParams escapes any problematic values
			return "?" + new URLSearchParams(data).toString();
		}
		return "";
	}

	/** @deprecated Use get/post/del instead! */
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
			try {
				err = error_msg(JSON.parse(error as string));
			} catch {
				err = error_msg(undefined);
			}
			throw error;
		}
	}

	/** Fetches the data, throwing an exception if something went wrong */
	export async function get<T>(url: string, query: Record<string, any>): Promise<T> {
		try {
			let response = await fetch(url + query_str(query), { method: "GET" });
			if (response.ok) return (await response.json()) as T;

			throw await response.json();
		} catch (error) {
			if (dialog) dialog.open();
			err = error_msg(error);
			throw error;
		}
	}

	/** Posts/updates the data, throwing an exception if something went wrong */
	export async function post(data: any, url: string, query: Record<string, any>): Promise<void> {
		try {
			let response = await fetch(url + query_str(query), {
				method: "POST",
				headers: {
					"Content-Type": "application/json; charset=utf-8"
				},
				body: JSON.stringify(data)
			});
			if (response.ok) return;

			throw await response.json();
		} catch (error) {
			if (dialog) dialog.open();
			err = error_msg(error);
			throw error;
		}
	}

	/** Deletes the data, throwing an exception if something went wrong */
	export async function del(url: string, query: Record<string, any>): Promise<void> {
		try {
			let response = await fetch(url + query_str(query), { method: "DELETE" });
			if (response.ok) return;

			throw await response.json();
		} catch (error) {
			if (dialog) dialog.open();
			err = error_msg(error);
			throw error;
		}
	}

	function error_msg(string: any): string {
		switch (string) {
			case "Arguments":
				return ".error.input";
			case "Logic":
				return ".error.update";
			case "FileNotFound":
				return ".error.file-open";
			case "FileOpen":
				return ".error.file-open";
			case "SQL":
				return ".error.sql";
			case "Network":
				return ".error.network";
			case "InvalidFormat":
				return ".error.format";
			case "NothingFound":
				return ".error.none";
			case "InvalidBook":
				return ".book.invalid";
			case "InvalidUser":
				return ".user.invalid";
			case "LendingUserMayNotBorrow":
				return ".error.lending.user";
			case "LendingBookNotBorrowable":
				return ".error.lending.book";
			case "LendingBookAlreadyBorrowed":
				return ".error.lending.already-borrowed";
			case "LendingBookAlreadyBorrowedByUser":
				return ".error.lending.already-borrowed-by";
			case "LendingBookNotBorrowed":
				return ".error.lending.not-borrowed";
			case "LendingBookAlreadyReserved":
				return ".error.lending.already-reserved";
			case "UnsupportedProjectVersion":
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
