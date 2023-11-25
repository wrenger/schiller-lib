import { _ } from "svelte-i18n";

namespace api {
	export interface About {
		name: string;
		version: string;
		repository: string;
		authors: string;
		description: string;
		license: string;
	}

	export interface Stats {
		books: number;
		authors: number;
		users: number;
		borrows: number;
		reservations: number;
		overdues: number;
	}

	export interface Session {
		id: string;
		username: string;
	}

	export interface Settings {
		// Borrowing
		borrowing_duration: number;
		// DNB
		dnb_token: string;
		// Mail
		mail_last_reminder: string;
		mail_from: string;
		mail_host: string;
		mail_password: string;
		// Mail Templates
		mail_info_subject: string;
		mail_info_content: string;
		mail_overdue_subject: string;
		mail_overdue_content: string;
		mail_overdue2_subject: string;
		mail_overdue2_content: string;
	}

	export interface Limited<T> {
		total_count: number;
		rows: T[];
	}

	export interface Book {
		id: string;
		isbn: string;
		title: string;
		publisher: string;
		year: number;
		costs: number;
		note: string;
		borrowable: boolean;
		category: string;
		authors: string[];
		borrower: string;
		deadline: string;
		reservation: string;
	}

	export type BookState = "None" | "Borrowable" | "NotBorrowable" | "Borrowed" | "Reserved";

	export interface BookSearch {
		query?: string;
		category?: string;
		state?: BookState;
		offset?: number;
		limit?: number;
	}

	export interface User {
		account: string;
		forename: string;
		surname: string;
		role: string;
		may_borrow: boolean;
	}

	export interface UserSearch {
		query?: string;
		may_borrow?: boolean;
		offset?: number;
		limit?: number;
	}

	export interface Category {
		id: string;
		name: string;
		section: string;
	}

	export interface MailBody {
		account: string;
		subject: string;
		body: string;
	}

	export type QueryParam = Record<string, any>;

	export function keys<T extends object>(obj: T) {
		return Object.keys(obj) as Array<keyof T>;
	}

	// -------------------------------------------------------------------------
	// General
	// -------------------------------------------------------------------------

	export async function about(): Promise<About> {
		return get("api/about");
	}
	export async function stats(): Promise<Stats> {
		return get("api/stats");
	}
	export async function session(): Promise<Session> {
		return get("api/session");
	}

	export async function settings(): Promise<Settings> {
		return get("api/settings");
	}
	export async function settings_update(settings: Partial<Settings>) {
		await post(settings, "api/settings");
	}

	// -------------------------------------------------------------------------
	// Book
	// -------------------------------------------------------------------------

	export async function book_search(query: BookSearch): Promise<Limited<Book>> {
		return get("api/book", query);
	}
	export async function book_add(book: Book) {
		await post(book, "api/book");
	}
	export async function book(id: string): Promise<Book> {
		return get("api/book/" + encodeURIComponent(id));
	}
	export async function book_update(id: string, book: Book) {
		await post(book, "api/book/" + encodeURIComponent(id));
	}
	export async function book_delete(id: string) {
		await del("api/book/" + encodeURIComponent(id));
	}
	export async function book_id(book: Book): Promise<string> {
		return post_get(book, "api/book-id");
	}
	export async function book_fetch(isbn: string): Promise<Partial<Book>> {
		return get("api/book-fetch/" + encodeURIComponent(isbn));
	}

	// -------------------------------------------------------------------------
	// User
	// -------------------------------------------------------------------------

	export async function user_search(query: UserSearch): Promise<Limited<User>> {
		return get("api/user", query);
	}
	export async function user_add(user: User) {
		await post(user, "api/user");
	}
	export async function user(account: string): Promise<User> {
		return get("api/user/" + encodeURIComponent(account));
	}
	export async function user_update(account: string, user: User) {
		await post(user, "api/user/" + encodeURIComponent(account));
	}
	export async function user_delete(account: string) {
		await del("api/user/" + encodeURIComponent(account));
	}
	export async function user_fetch(account: string): Promise<User> {
		return get("api/user-fetch/" + encodeURIComponent(account));
	}
	export async function user_update_roles() {
		await post({}, "api/user-update-roles");
	}

	// -------------------------------------------------------------------------
	// Category
	// -------------------------------------------------------------------------

	export async function categories(): Promise<Category[]> {
		return get("api/category");
	}
	export async function category_add(category: Category) {
		await post(category, "api/category");
	}
	export async function category_update(id: string, category: Category) {
		await post(category, "api/category/" + encodeURIComponent(id));
	}
	export async function category_delete(id: string) {
		await del("api/category/" + encodeURIComponent(id));
	}

	// -------------------------------------------------------------------------
	// Lending
	// -------------------------------------------------------------------------

	export async function lend(id: string, account: string, deadline: string): Promise<Book> {
		return post_get({}, "api/lending/lend", { id, account, deadline });
	}
	export async function return_back(id: string): Promise<Book> {
		return post_get({}, "api/lending/return", { id });
	}
	export async function reserve(id: string, account: string): Promise<Book> {
		return post_get({}, "api/lending/reserve", { id, account });
	}
	export async function release(id: string): Promise<Book> {
		return post_get({}, "api/lending/release", { id });
	}

	// -------------------------------------------------------------------------
	// Mail
	// -------------------------------------------------------------------------

	export async function mail(mails: MailBody[]) {
		await post(mails, "api/notify");
	}

	// -------------------------------------------------------------------------
	// Overdues
	// -------------------------------------------------------------------------

	export async function overdues(): Promise<[Book, User][]> {
		return get("api/overdues");
	}

	/** Fetches the data, throwing an exception if something went wrong */
	async function get<T>(url: string, query: QueryParam = {}): Promise<T> {
		let response = await fetch(url + query_str(query), { method: "GET" });
		if (response.ok) return (await response.json()) as T;

		error(await response.json());
	}

	/** Posts/updates the data, throwing an exception if something went wrong */
	async function post(data: any, url: string, query: QueryParam = {}) {
		let response = await fetch(url + query_str(query), {
			method: "POST",
			headers: {
				"Content-Type": "application/json; charset=utf-8"
			},
			body: JSON.stringify(data)
		});
		if (response.ok) return;

		error(await response.json());
	}

	/** Posts/updates the data, throwing an exception if something went wrong */
	async function post_get<T>(data: any, url: string, query: QueryParam = {}): Promise<T> {
		let response = await fetch(url + query_str(query), {
			method: "POST",
			headers: {
				"Content-Type": "application/json; charset=utf-8"
			},
			body: JSON.stringify(data)
		});
		if (response.ok) return response.json();

		error(await response.json());
	}

	/** Deletes the data, throwing an exception if something went wrong */
	async function del(url: string, query: QueryParam = {}) {
		let response = await fetch(url + query_str(query), { method: "DELETE" });
		if (response.ok) return;

		error(await response.json());
	}

	/** Safely create a valid query string from the provided query parameters */
	function query_str(params: QueryParam): string {
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

	/** For api errors, Opens a modal */
	function error(error: string): never {
		const modal: HTMLDialogElement | null = document.getElementById(
			"error-modal"
		) as HTMLDialogElement | null;

		if (modal) {
			const errorTextElement: HTMLParagraphElement | null = modal.querySelector(".card-body p");
			if (errorTextElement) {
				let errorLocalized: string = "";
				_.subscribe((_) => (errorLocalized = _(error_msg(error))));
				errorTextElement.textContent = errorLocalized;
			}

			modal.showModal();
		}

		throw error;
	}

	/** Server Error translations */
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
}

export default api;
