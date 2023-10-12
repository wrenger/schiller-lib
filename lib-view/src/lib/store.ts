import { writable } from "svelte/store";

interface LocalSettings {
	borrowing_time: number;
	separator: string;
	dnb: string;
	host: string;
	sender: string;
	password: string;
	title1: string;
	text1: string;
	title2: string;
	text2: string;
	title3: string;
	text3: string;
}

export const settingsLocal = writable<LocalSettings>({
	borrowing_time: 0,
	separator: "",
	dnb: "",
	host: "",
	sender: "",
	password: "",
	title1: "",
	text1: "",
	title2: "",
	text2: "",
	title3: "",
	text3: ""
});
