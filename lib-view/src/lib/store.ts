import { DateTime } from 'luxon';
import type api from './api';
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

export const settingsGlobal = persisted<GlobalSettings>('settings-global', {
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

interface Category {
	id: string;
	name: string;
	section: string;
}

export const categories = persisted<Category[]>('categories', []);

export const stats = writable<api.Stats>({
	books: 0,
	users: 0,
	categories: 0,
	borrows: 0,
	reservations: 0,
	overdues: 0
});
