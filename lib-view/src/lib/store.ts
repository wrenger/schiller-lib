import { DateTime } from 'luxon';
import { writable } from 'svelte/store';
import type api from './api';

export interface GlobalSettings {
	borrowing_duration: number;
	dnb_token: string;
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
	dnb_token: '',
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

export const category = writable<Category[]>([]);

interface ErrorStore {
	message: string | undefined;
}

export const errorStore = writable<ErrorStore>({ message: undefined });

export const state = writable<{}>({});
