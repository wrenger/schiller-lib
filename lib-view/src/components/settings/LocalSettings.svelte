<script lang="ts">
	import { _ } from "svelte-i18n";
	import { get, set } from "../../lib/util";
	import { setLang } from "$lib/i18n";

	set(
		"data-bs-theme",
		localStorage.getItem("data-bs-theme")
			? (localStorage.getItem("data-bs-theme") as string)
			: "dark"
	);
	set("lang", localStorage.getItem("lang") ? (localStorage.getItem("lang") as string) : "en");

	let theme = get("data-bs-theme");
	let lang = get("lang");

	setLang(lang);

	let initialSettings = {
		theme,
		lang
	};

	export function save() {
		set("data-bs-theme", theme);
		set("lang", lang);
		setLang(lang);
		initialSettings = {
			theme,
			lang
		};
	}

	export function cancel() {
		theme = initialSettings.theme;
		lang = initialSettings.lang;
	}
</script>

<h5 class="mb-2 mt-2">{$_(".pref.appearance.title")}</h5>
<div class="form-check">
	<input
		bind:group={theme}
		class="form-check-input"
		type="radio"
		name="color-toggle"
		id="light-toggle"
		value="light"
	/>
	<label class="form-check-label" for="light-toggle">{$_(".pref.appearance.light")}</label>
</div>
<div class="form-check">
	<input
		bind:group={theme}
		class="form-check-input"
		type="radio"
		name="color-toggle"
		id="dark-toggle"
		value="dark"
	/>
	<label class="form-check-label" for="dark-toggle">{$_(".pref.appearance.dark")}</label>
</div>
<h5 class="mb-2 mt-2">{$_(".lang.title")}</h5>
<div class="form-check">
	<input
		bind:group={lang}
		class="form-check-input"
		type="radio"
		name="lang-toggle"
		id="en-toggle"
		value="en"
	/>
	<label class="form-check-label" for="en-toggle">{$_(".lang.en")}</label>
</div>
<div class="form-check">
	<input
		bind:group={lang}
		class="form-check-input"
		type="radio"
		name="lang-toggle"
		id="ger-toggle"
		value="ger"
	/>
	<label class="form-check-label" for="ger-toggle">{$_(".lang.de")}</label>
</div>
