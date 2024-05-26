import { _ } from 'svelte-i18n';
import Ajv, { type JTDParser, type JTDSchemaType } from 'ajv/dist/jtd';
import { toast } from 'svelte-sonner';

namespace api {
	const ajv = new Ajv();

	export interface About {
		name: string;
		version: string;
		repository: string;
		authors: string[];
		description: string;
		license: string;
	}
	const parse_about = ajv.compileParser<About>({
		properties: {
			name: { type: 'string' },
			version: { type: 'string' },
			repository: { type: 'string' },
			authors: { elements: { type: 'string' } },
			description: { type: 'string' },
			license: { type: 'string' }
		}
	});

	export interface Stats {
		books: number;
		users: number;
		categories: number;
		borrows: number;
		reservations: number;
		overdues: number;
	}
	const parse_stats = ajv.compileParser<Stats>({
		properties: {
			books: { type: 'uint32' },
			users: { type: 'uint32' },
			categories: { type: 'uint32' },
			borrows: { type: 'uint32' },
			reservations: { type: 'uint32' },
			overdues: { type: 'uint32' }
		}
	});

	export interface Session {
		id: string;
		username: string;
	}
	const parse_session = ajv.compileParser<Session>({
		properties: {
			id: { type: 'string' },
			username: { type: 'string' }
		}
	});

	export interface MailTemplate {
		subject: string;
		body: string;
	}
	const schema_mail_template: JTDSchemaType<MailTemplate> = {
		properties: {
			subject: { type: 'string' },
			body: { type: 'string' }
		}
	};

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
		mail_info: MailTemplate;
		mail_overdue: MailTemplate;
		mail_overdue2: MailTemplate;
	}
	const parse_settings = ajv.compileParser<Settings>({
		properties: {
			borrowing_duration: { type: 'uint32' },
			dnb_token: { type: 'string' },
			mail_last_reminder: { type: 'string' },
			mail_from: { type: 'string' },
			mail_host: { type: 'string' },
			mail_password: { type: 'string' },
			mail_info: schema_mail_template,
			mail_overdue: schema_mail_template,
			mail_overdue2: schema_mail_template
		}
	});

	export interface Borrower {
		user: string;
		deadline: string;
	}

	export interface Book {
		id: string;
		isbn: string;
		title: string;
		publisher: string;
		year: number;
		costs: number;
		note?: string;
		borrowable: boolean;
		category: string;
		authors: string;
		borrower?: Borrower;
		reservation?: string;
	}
	const schema_book: JTDSchemaType<Book> = {
		properties: {
			id: { type: 'string' },
			isbn: { type: 'string' },
			title: { type: 'string' },
			publisher: { type: 'string' },
			year: { type: 'uint32' },
			costs: { type: 'float32' },
			borrowable: { type: 'boolean' },
			category: { type: 'string' },
			authors: { type: 'string' }
		},
		optionalProperties: {
			note: { type: 'string' },
			reservation: { type: 'string' },
			borrower: {
				properties: {
					user: { type: 'string' },
					deadline: { type: 'string' }
				}
			}
		}
	};

	const parse_book = ajv.compileParser(schema_book);
	const parse_partial_book = ajv.compileParser<Partial<Book>>({
		optionalProperties: {
			...schema_book.properties,
			...schema_book.optionalProperties
		}
	});

	export type BookState = 'None' | 'Borrowable' | 'NotBorrowable' | 'Borrowed' | 'Reserved';

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
	function userDef(u: Partial<User>): User {
		return {
			account: u.account ?? '',
			forename: u.forename ?? '',
			surname: u.surname ?? '',
			role: u.role ?? '',
			may_borrow: u.may_borrow ?? true
		};
	}
	const schema_user: JTDSchemaType<Partial<User>> = {
		optionalProperties: {
			account: { type: 'string' },
			forename: { type: 'string' },
			surname: { type: 'string' },
			role: { type: 'string' },
			may_borrow: { type: 'boolean' }
		}
	};
	const parse_user = ajv.compileParser(schema_user);

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
	const schema_category: JTDSchemaType<Category> = {
		properties: {
			id: { type: 'string' },
			name: { type: 'string' },
			section: { type: 'string' }
		}
	};
	const parse_categories = ajv.compileParser<Category[]>({
		elements: schema_category
	});

	export interface MailBody {
		account: string;
		subject: string;
		body: string;
	}

	export interface Limited<T> {
		total: number;
		rows: T[];
	}
	const parse_limited_books = ajv.compileParser<Limited<Book>>({
		properties: {
			total: { type: 'uint32' },
			rows: { elements: schema_book }
		}
	});
	const parse_limited_users = ajv.compileParser<Limited<Partial<User>>>({
		properties: {
			total: { type: 'uint32' },
			rows: { elements: schema_user }
		}
	});

	export interface Overdue {
		book: Book;
		user: User;
	}
	const parse_overdues = ajv.compileParser<{ book: Book; user: Partial<User> }[]>({
		elements: {
			properties: {
				book: schema_book,
				user: schema_user
			}
		}
	});

	export type QueryParam = Record<string, any>;

	export function keys<T extends object>(obj: T) {
		return Object.keys(obj) as Array<keyof T>;
	}

	// -------------------------------------------------------------------------
	// General
	// -------------------------------------------------------------------------

	export async function about(): Promise<About> {
		return get('api/about', parse_about);
	}
	export async function stats(): Promise<Stats> {
		return get('api/stats', parse_stats);
	}
	export async function session(): Promise<Session> {
		return get('api/session', parse_session);
	}

	export async function settings(): Promise<Settings> {
		return get('api/settings', parse_settings);
	}
	export async function settings_update(settings: Partial<Settings>) {
		await post('api/settings', settings);
	}

	// -------------------------------------------------------------------------
	// Book
	// -------------------------------------------------------------------------

	export async function book_search(query: BookSearch): Promise<Limited<Book>> {
		return get('api/book', parse_limited_books, query);
	}
	export async function book_add(book: Book) {
		await post('api/book', book);
	}
	export async function book(id: string): Promise<Book> {
		return get('api/book/' + encodeURIComponent(id), parse_book);
	}
	export async function book_update(id: string, book: Book) {
		await post('api/book/' + encodeURIComponent(id), book);
	}
	export async function book_delete(id: string) {
		await del('api/book/' + encodeURIComponent(id));
	}
	export async function book_id(book: Book): Promise<string> {
		const parse_book_id = ajv.compileParser<string>({ type: 'string' });
		return post_get('api/book-id', book, parse_book_id);
	}
	export async function book_fetch(isbn: string): Promise<Partial<Book>> {
		return get('api/book-fetch/' + encodeURIComponent(isbn), parse_partial_book);
	}

	// -------------------------------------------------------------------------
	// User
	// -------------------------------------------------------------------------

	export async function user_search(query: UserSearch): Promise<Limited<User>> {
		return get('api/user', parse_limited_users, query).then((l) => ({
			total: l.total,
			rows: l.rows.map(userDef)
		}));
	}
	export async function user_add(user: User) {
		await post('api/user', user);
	}
	export async function user(account: string): Promise<User> {
		return get('api/user/' + encodeURIComponent(account), parse_user).then(userDef);
	}
	export async function user_update(account: string, user: User) {
		await post('api/user/' + encodeURIComponent(account), user);
	}
	export async function user_delete(account: string) {
		await del('api/user/' + encodeURIComponent(account));
	}
	export async function user_fetch(account: string): Promise<User> {
		return get('api/user-fetch/' + encodeURIComponent(account), parse_user).then(userDef);
	}
	export async function user_update_roles() {
		await post('api/user-update-roles', {});
	}

	// -------------------------------------------------------------------------
	// Category
	// -------------------------------------------------------------------------

	export async function categories(): Promise<Category[]> {
		return get('api/category', parse_categories);
	}
	export async function category_add(category: Category) {
		await post('api/category', category);
	}
	export async function category_update(id: string, category: Category) {
		await post('api/category/' + encodeURIComponent(id), category);
	}
	export async function category_delete(id: string) {
		await del('api/category/' + encodeURIComponent(id));
	}

	// -------------------------------------------------------------------------
	// Lending
	// -------------------------------------------------------------------------

	export async function lend(id: string, account: string, deadline: string | null): Promise<Book> {
		return post_get('api/lending/lend', {}, parse_book, { id, account, deadline });
	}
	export async function return_back(id: string): Promise<Book> {
		return post_get('api/lending/return', {}, parse_book, { id });
	}
	export async function reserve(id: string, account: string): Promise<Book> {
		return post_get('api/lending/reserve', {}, parse_book, { id, account });
	}
	export async function release(id: string): Promise<Book> {
		return post_get('api/lending/release', {}, parse_book, { id });
	}

	// -------------------------------------------------------------------------
	// Mail
	// -------------------------------------------------------------------------

	export async function mail(mails: MailBody[]) {
		await post('api/notify', mails);
	}

	// -------------------------------------------------------------------------
	// Overdues
	// -------------------------------------------------------------------------

	export async function overdues(): Promise<Overdue[]> {
		return get('api/overdues', parse_overdues).then((o) =>
			o.map((e) => ({
				book: e.book,
				user: userDef(e.user)
			}))
		);
	}

	/** Fetches the data, throwing an exception if something went wrong */
	async function get<T>(url: string, parse: JTDParser<T>, query: QueryParam = {}): Promise<T> {
		let response = await fetch(url + query_str(query), { method: 'GET' });
		if (response.ok) {
			let result = parse(await response.text());
			if (result) return result;
			console.error(parse.message);
			error('InvalidFormat');
		}
		error(await response.json());
	}

	/** Posts/updates the data, throwing an exception if something went wrong */
	async function post(url: string, data: any, query: QueryParam = {}) {
		let response = await fetch(url + query_str(query), {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json; charset=utf-8'
			},
			body: JSON.stringify(data)
		});
		if (response.ok) return;

		error(await response.json());
	}

	/** Posts/updates the data, throwing an exception if something went wrong */
	async function post_get<T>(
		url: string,
		data: any,
		parse: JTDParser<T>,
		query: QueryParam = {}
	): Promise<T> {
		let response = await fetch(url + query_str(query), {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json; charset=utf-8'
			},
			body: JSON.stringify(data)
		});
		if (response.ok) {
			let result = parse(await response.text());
			if (result) return result;
			console.error(parse.message);
			error('InvalidFormat');
		}

		error(await response.json());
	}

	/** Deletes the data, throwing an exception if something went wrong */
	async function del(url: string, query: QueryParam = {}) {
		let response = await fetch(url + query_str(query), { method: 'DELETE' });
		if (response.ok) return;

		error(await response.json());
	}

	/** Safely create a valid query string from the provided query parameters */
	function query_str(params: QueryParam): string {
		if (params) {
			let data: Record<string, string> = {};
			for (let key in params) {
				if (params[key] != null) data[key] = params[key].toString();
			}
			// the URLSearchParams escapes any problematic values
			return '?' + new URLSearchParams(data).toString();
		}
		return '';
	}

	/** For api errors, Opens a toast */
	function error(error: string): never {
		let errorLocalized: string = '';
		_.subscribe((_) => (errorLocalized = _(error_msg(error))));

		toast.error(errorLocalized);

		throw error;
	}

	/** Server Error translations */
	export function error_msg(string: any): string {
		switch (string) {
			case 'Arguments':
				return '.error.input';
			case 'FileOpen':
				return '.error.file-open';
			case 'Network':
				return '.error.network';
			case 'InvalidFormat':
				return '.error.format';
			case 'NothingFound':
				return '.error.none';
			case 'ReferencedUser':
				return '.user.referenced.del';
			case 'ReferencedCategory':
				return '.category.not-empty.del';
			case 'InvalidBook':
				return '.book.invalid';
			case 'InvalidUser':
				return '.user.invalid';
			case 'LendingUserMayNotBorrow':
				return '.error.lending.user';
			case 'LendingBookNotBorrowable':
				return '.error.lending.book';
			case 'LendingBookAlreadyBorrowed':
				return '.error.lending.already-borrowed';
			case 'LendingBookAlreadyBorrowedByUser':
				return '.error.lending.already-borrowed-by';
			case 'LendingBookNotBorrowed':
				return '.error.lending.not-borrowed';
			case 'LendingBookAlreadyReserved':
				return '.error.lending.already-reserved';
			case 'LendingBookNotReserved':
				return '.error.lending.not-reserved';
			case 'UnsupportedProjectVersion':
				return '.error.update';
			default:
				return '.error.unknown';
		}
	}

	/** Replaces the placeholders in the mail templates */
	export function mail_replace(
		template: MailTemplate,
		booktitle: string,
		username: string
	): MailTemplate {
		return {
			subject: template.subject
				.replaceAll('{booktitle}', booktitle)
				.replaceAll('{username}', username),
			body: template.body.replaceAll('{booktitle}', booktitle).replaceAll('{username}', username)
		};
	}
}

export default api;
