const BASE = '/api';

namespace api {
    export interface About {
        name: string;
        version: string;
        repository: string;
        authors: string[];
        description: string;
        license: string;
    }

    /**
        Data object for book.
    */
    export interface Book {
        id: string;
        isbn: string;
        title: string;
        publisher: string;
        year: number;
        costs: number;
        note: string | null;
        borrowable: boolean;
        category: string;
        authors: string;
        borrower: Borrower | null;
        reservation: string | null;
    }

    export interface BookData {
        title: string;
        authors: string[];
        publisher: string;
        costs: number;
    }

    /**
        Book search parameters
    */
    export interface BookSearch {
        query: string;
        category: string;
        state: BookState;
        offset: number;
        limit: number;
    }

    export interface Borrower {
        user: string;
        deadline: string;
    }

    /**
        Data object for categories
    */
    export interface Category {
        id: string;
        name: string;
        section: string;
    }

    export interface LendParams {
        id: string;
        account: string;
        /**
            ISO date format: YYYY-MM-DD
        */
        deadline: string;
    }

    /**
        Search result containing the total number of found records.
    */
    export interface Limited<T> {
        /**
            Total number of results (without limit)
        */
        total: number;
        rows: T[];
    }

    /**
        The user data we'll get back from oauth.
        
        E.g. Discord: https://discord.com/developers/docs/resources/user#user-object-user-structure
    */
    export interface Login {
        id: string;
        username: string;
    }

    /**
        Template for a mail notification
    */
    export interface MailTemplate {
        subject: string;
        body: string;
    }

    export interface Message {
        account: string;
        subject: string;
        body: string;
    }

    /**
        Borrowed books that missed the deadline
    */
    export interface Overdue {
        book: Book;
        user: User;
    }

    export interface ReserveParams {
        id: string;
        account: string;
    }

    export interface ReturnParams {
        id: string;
    }

    /**
        Library settings
    */
    export interface Settings {
        borrowing_duration: number;
        dnb_token: string;
        mail_last_reminder: string;
        mail_from: string;
        mail_host: string;
        mail_password: string;
        mail_info: MailTemplate;
        mail_overdue: MailTemplate;
        mail_overdue2: MailTemplate;
    }

    /**
        Data object for book.
    */
    export interface Stats {
        books: number;
        users: number;
        categories: number;
        borrows: number;
        reservations: number;
        overdues: number;
    }

    /**
        Data object for a user.
    */
    export interface User {
        account: string;
        forename: string;
        surname: string;
        role: string | null;
        may_borrow: boolean | null;
    }

    /**
        Parameters for the normal search
    */
    export interface UserSearch {
        query: string;
        may_borrow: boolean | null;
        offset: number;
        limit: number;
    }

    /**
        Borrow status of a book
    */
    export enum BookState {
        /**
            No status
        */
        None = "None",
        /**
            Can be borrowed
        */
        Borrowable = "Borrowable",
        /**
            Cannot be borrowed
        */
        NotBorrowable = "NotBorrowable",
        /**
            Is already borrowed
        */
        Borrowed = "Borrowed",
        /**
            Is already reserved
        */
        Reserved = "Reserved",
    }

    /**
        The api compatible error type.
        On the frontend there are specific error messages displayed for each of the error types.
        
        More specific error messages are removed to be api compatible.
        Those messages are logged however.
    */
    export enum Error {
        /**
            The user provided arguments are malformed
        */
        Arguments = "Arguments",
        /**
            A file could not be found or opened
        */
        FileOpen = "FileOpen",
        /**
            Could not connect to server
        */
        Network = "Network",
        /**
            Invalid file format
        */
        InvalidFormat = "InvalidFormat",
        /**
            No matching results
        */
        NothingFound = "NothingFound",
        /**
            Deletion not possible as the user is still referenced
        */
        ReferencedUser = "ReferencedUser",
        /**
            Deletion not possible as the category is still referenced
        */
        ReferencedCategory = "ReferencedCategory",
        /**
            The book has invalid or missing fields
        */
        InvalidBook = "InvalidBook",
        /**
            The user has invalid or missing fields
        */
        InvalidUser = "InvalidUser",
        /**
            User may not borrow
        */
        LendingUserMayNotBorrow = "LendingUserMayNotBorrow",
        /**
            Book cannot be borrowed
        */
        LendingBookNotBorrowable = "LendingBookNotBorrowable",
        /**
            Book is already borrowed
        */
        LendingBookAlreadyBorrowed = "LendingBookAlreadyBorrowed",
        /**
            Book cannot be reserved as the user already borrows it
        */
        LendingBookAlreadyBorrowedByUser = "LendingBookAlreadyBorrowedByUser",
        /**
            The book cannot be reserved or returned as it is not borrowed
        */
        LendingBookNotBorrowed = "LendingBookNotBorrowed",
        /**
            The book is already reserved
        */
        LendingBookAlreadyReserved = "LendingBookAlreadyReserved",
        /**
            The book is not reserved
        */
        LendingBookNotReserved = "LendingBookNotReserved",
        /**
            The database version is too old
        */
        UnsupportedProjectVersion = "UnsupportedProjectVersion",
    }

    /**
        Result type using the api error.
    */
    export type Result<T> = T | Error;

    async function fetch_api(endpoint: string, options: RequestInit): Promise<any> {
        const response = await fetch(endpoint, {
            headers: {
                "Content-Type": "application/json",
                ...options.headers,
            },
            ...options,
        });
        if (response.headers.get('Content-Length') === '0') {
			return;
		} else {
			return response.json();
		}
    }

    function query_str(params: Record<string, any>): string {
		if (params) {
			let data: Record<string, string> = {};
			for (let key in params) {
				if (params[key] != null) data[key] = params[key].toString();
			}
			return '?' + new URLSearchParams(data).toString();
		}
		return '';
	}

    /**
        Returns info about this project.
    */
    export async function about(): Promise<About> {
        return fetch_api(`${BASE}/about`, {
            method: "GET", 
        });
    }

    /**
        Adds a new book.
    */
    export async function book_add(data: Book): Promise<Result<Book>> {
        return fetch_api(`${BASE}/book`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Deletes the book including the its authors.
        Also borrowers & reservations for this book are removed.
    */
    export async function book_delete(path: string): Promise<Result<void>> {
        return fetch_api(`${BASE}/book/${encodeURIComponent(path)}`, {
            method: "DELETE", 
        });
    }

    /**
        Returns the book with the given `id`.
    */
    export async function book_fetch(path: string): Promise<Result<Book>> {
        return fetch_api(`${BASE}/book/${encodeURIComponent(path)}`, {
            method: "GET", 
        });
    }

    /**
        Fetch the data of the book from the DNB an their like.
    */
    export async function book_fetch_data(path: string): Promise<Result<BookData>> {
        return fetch_api(`${BASE}/book-fetch/${encodeURIComponent(path)}`, {
            method: "GET", 
        });
    }

    /**
        Generates a new book id.
    */
    export async function book_generate_id(data: Book): Promise<Result<string>> {
        return fetch_api(`${BASE}/book-id`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Preforms a simple media search with the given `query`.
    */
    export async function book_search(query: BookSearch): Promise<Result<Limited<Book>>> {
        return fetch_api(`${BASE}/book${query_str(query)}`, {
            method: "GET", 
        });
    }

    /**
        Updates the book and all references if its id changes.
    */
    export async function book_update(path: string, data: Book): Promise<Result<Book>> {
        return fetch_api(`${BASE}/book/${encodeURIComponent(path)}`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Adds a new category.
    */
    export async function category_add(data: Category): Promise<Result<Category>> {
        return fetch_api(`${BASE}/category`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Removes the category or returns a `Error::StillReferenced` if it is still in use.
    */
    export async function category_delete(path: string): Promise<Result<void>> {
        return fetch_api(`${BASE}/category/${encodeURIComponent(path)}`, {
            method: "DELETE", 
        });
    }

    /**
        Fetches and returns all categories.
    */
    export async function category_list(): Promise<Result<Category[]>> {
        return fetch_api(`${BASE}/category`, {
            method: "GET", 
        });
    }

    /**
        Returns the number of books in this category.
    */
    export async function category_references(path: string): Promise<Result<number>> {
        return fetch_api(`${BASE}/category-refs/${encodeURIComponent(path)}`, {
            method: "GET", 
        });
    }

    /**
        Updates the category and all references.
    */
    export async function category_update(path: string, data: Category): Promise<Result<Category>> {
        return fetch_api(`${BASE}/category/${encodeURIComponent(path)}`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Lends the book to the specified user.
    */
    export async function lending_lend(query: LendParams): Promise<Result<Book>> {
        return fetch_api(`${BASE}/lending/lend${query_str(query)}`, {
            method: "POST", 
        });
    }

    /**
        Returns the list of expired borrowing periods.
    */
    export async function lending_overdues(): Promise<Result<Overdue[]>> {
        return fetch_api(`${BASE}/overdues`, {
            method: "GET", 
        });
    }

    /**
        Removes the reservation from the specified book.
    */
    export async function lending_release(query: ReturnParams): Promise<Result<Book>> {
        return fetch_api(`${BASE}/lending/release${query_str(query)}`, {
            method: "POST", 
        });
    }

    /**
        Creates a reservation for the borrowed book.
    */
    export async function lending_reserve(query: ReserveParams): Promise<Result<Book>> {
        return fetch_api(`${BASE}/lending/reserve${query_str(query)}`, {
            method: "POST", 
        });
    }

    /**
        Returns the book.
    */
    export async function lending_return(query: ReturnParams): Promise<Result<Book>> {
        return fetch_api(`${BASE}/lending/return${query_str(query)}`, {
            method: "POST", 
        });
    }

    export async function mail_notify(data: Message[]): Promise<Result<void>> {
        return fetch_api(`${BASE}/notify`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Returns the project statistics.
    */
    export async function session(): Promise<Result<Login>> {
        return fetch_api(`${BASE}/session`, {
            method: "GET", 
        });
    }

    /**
        Returns the project settings.
        They are fetched when opening a project, so that this function only
        returns copies of the cached version.
    */
    export async function settings_get(): Promise<Result<Settings>> {
        return fetch_api(`${BASE}/settings`, {
            method: "GET", 
        });
    }

    /**
        Updates project settings.
    */
    export async function settings_update(data: Settings): Promise<Result<void>> {
        return fetch_api(`${BASE}/settings`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Returns the project statistics.
    */
    export async function stats(): Promise<Result<Stats>> {
        return fetch_api(`${BASE}/stats`, {
            method: "GET", 
        });
    }

    /**
        Adds a new user.
    */
    export async function user_add(data: User): Promise<Result<User>> {
        return fetch_api(`${BASE}/user`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Deletes the user.
        
        Returns a `Error::StillReferenced` if there are any borrows or reservations left.
    */
    export async function user_delete(path: string): Promise<Result<void>> {
        return fetch_api(`${BASE}/user/${encodeURIComponent(path)}`, {
            method: "DELETE", 
        });
    }

    /**
        Returns the user with the given `account`.
    */
    export async function user_fetch(path: string): Promise<Result<User>> {
        return fetch_api(`${BASE}/user/${encodeURIComponent(path)}`, {
            method: "GET", 
        });
    }

    /**
        Fetch the data of the user from the specified user file.
    */
    export async function user_fetch_data(path: string): Promise<Result<User>> {
        return fetch_api(`${BASE}/user-fetch/${encodeURIComponent(path)}`, {
            method: "GET", 
        });
    }

    /**
        Performs a simple user search with the given `text`.
    */
    export async function user_search(query: UserSearch): Promise<Result<Limited<User>>> {
        return fetch_api(`${BASE}/user${query_str(query)}`, {
            method: "GET", 
        });
    }

    /**
        Updates the user and all references if its account changes.
    */
    export async function user_update(path: string, data: User): Promise<Result<User>> {
        return fetch_api(`${BASE}/user/${encodeURIComponent(path)}`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    /**
        Deletes the roles from all users and inserts the new roles.
        
        The roles of all users not contained in the given list are cleared.
    */
    export async function user_update_roles(): Promise<Result<void>> {
        return fetch_api(`${BASE}/user-update-roles`, {
            method: "POST", 
        });
    }
}

export default api;
