import { writable } from "svelte/store";

interface Lang {
	en: string;
	de: string;
}

export const translations = writable<Record<string, Lang>>({});

const csvUrl = "/translations.csv";

fetch(csvUrl)
	.then((response) => response.text())
	.then((csvText) => {
		const rows = csvText.split("\n");
		for (const row of rows) {
			const [key, en, de] = row.split(",");
			if (key && en && de) {
				translations.update((currentTranslations) => {
					currentTranslations[key] = { en, de };
					return currentTranslations;
				});
			}
		}
	})
	.catch((error) => {
		console.error("Error loading translations:", error);
	});

export type { Lang };
