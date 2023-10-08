// src/lib/i18n/index.ts
import { browser } from "$app/environment";
import { init, register, locale } from "svelte-i18n";

const defaultLocale = "en";

register("en", () => import("./locales/en.json"));
register("ger", () => import("./locales/de.json"));

init({
	fallbackLocale: defaultLocale,
	initialLocale: browser ? window.navigator.language : defaultLocale
});

export function setLang(lang: string) {
	locale.set(lang);
}
