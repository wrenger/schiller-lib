export const prerender = true;

import { browser } from '$app/environment';
import '$lib/i18n';
import { waitLocale } from 'svelte-i18n';
import type { LayoutLoad } from './$types';
import { setLang } from '$lib/i18n';

export const load: LayoutLoad = async () => {
	if (browser) {
		const storedLang = localStorage.getItem('lang');
		if (storedLang) {
			setLang(storedLang);
		} else {
			setLang('en');
		}
	}
	await waitLocale();
};
