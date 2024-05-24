// src/lib/i18n/index.ts
import { browser } from '$app/environment';
import { init, register, locale } from 'svelte-i18n';

const defaultLocale = 'en';

register('en', () => import('./locales/en.json'));
register('de', () => import('./locales/de.json'));

let lang: string | null | undefined = undefined;

async function initializeI18n() {
	let initialLocale = defaultLocale;
	if (browser) {
		const storedLang = localStorage.getItem('lang');
		if (storedLang) {
			initialLocale = storedLang;
		} else {
			setLang(initialLocale);
		}
	}
	await init({
		fallbackLocale: defaultLocale,
		initialLocale
	});

	locale.subscribe((s) => (lang = s));
}

initializeI18n();

export function setLang(lang: string | undefined) {
	if (browser) {
		localStorage.setItem('lang', lang || defaultLocale);
	}
	locale.set(lang);
}

export function getLang() {
	return lang;
}
