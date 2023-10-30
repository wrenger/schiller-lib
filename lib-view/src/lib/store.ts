import { writable } from "svelte/store";

interface GlobalSettings {
	borrowing_duration: number;
	dnb_token: string;
	mail_last_reminder: string;
	mail_from: string;
	mail_host: string;
	mail_password: string;
	mail_info_subject: string;
	mail_info_content: string;
	mail_overdue_subject: string;
	mail_overdue_content: string;
	mail_overdue2_subject: string;
	mail_overdue2_content: string;
}

export const settingsGlobal = writable<GlobalSettings>({
	borrowing_duration: 0,
	dnb_token: "",
	mail_last_reminder: "",
	mail_from: "",
	mail_host: "",
	mail_password: "",
	mail_info_subject: "",
	mail_info_content: "",
	mail_overdue_subject: "",
	mail_overdue_content: "",
	mail_overdue2_subject: "",
	mail_overdue2_content: ""
});

interface Category {
	id: string;
	name: string;
	section: string;
}

export const category = writable<Category[]>([]);
