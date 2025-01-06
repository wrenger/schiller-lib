import { DateTime } from 'luxon';
import api from './api';
import { persisted } from 'svelte-persisted-store';
import { writable } from "svelte/store";

export interface GlobalSettings {
	borrowing_duration: number;
	overdue_warning_delay: number;
	mail_last_reminder: DateTime;
	mail_from: string;
	mail_host: string;
	mail_password: string;
	mail_info: api.MailTemplate;
	mail_overdue: api.MailTemplate;
	mail_overdue2: api.MailTemplate;
}

export const settingsGlobal = writable<GlobalSettings>({
	borrowing_duration: 0,
	overdue_warning_delay: 0,
	mail_last_reminder: DateTime.now(),
	mail_from: '',
	mail_host: '',
	mail_password: '',
	mail_info: {
		subject: '',
		body: ''
	},
	mail_overdue: {
		subject: '',
		body: ''
	},
	mail_overdue2: {
		subject: '',
		body: ''
	}
});

/// UI state of the book tab
export interface BookState {
	search: Omit<api.BookSearch, "offset" | "limit">;
	scroll: number;
	active: api.Book | null;
}
export const bookState = writable<BookState>({
	search: {
		query: '',
		category: '',
		state: api.BookState.None
	},
	scroll: 0,
	active: null,
});

/// UI state of the user tab
export interface UserState {
	search: Omit<api.UserSearch, "offset" | "limit">;
	scroll: number;
	active: api.User | null;
}
export const userState = writable<UserState>({
	search: {
		query: '',
		may_borrow: null
	},
	scroll: 0,
	active: null,
});

interface Category {
	id: string;
	name: string;
	section: string;
}
export const categories = writable<Category[]>([]);

export const stats = writable<api.Stats>({
	books: 0,
	users: 0,
	categories: 0,
	borrows: 0,
	reservations: 0,
	overdues: 0
});

// TODO: Is there a better way to access this global dialog?
export const reminderDialog = writable<{ openDialog: () => void } | null>(null);
