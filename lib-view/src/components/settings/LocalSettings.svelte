<script lang="ts">
	import { get, set } from "../../lib/util";

	set(
		"data-bs-theme",
		localStorage.getItem("data-bs-theme")
			? (localStorage.getItem("data-bs-theme") as string)
			: "dark"
	);
	set("lang", localStorage.getItem("lang") ? (localStorage.getItem("lang") as string) : "en");

	let theme = get("data-bs-theme");
	let lang = get("lang");

	let initialSettings = {
		theme,
		lang
	};

	export function save() {
		set("data-bs-theme", theme);
		set("lang", lang);
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

<h5 class="mb-2 mt-2">Theme</h5>
<div class="form-check">
	<input
		bind:group={theme}
		class="form-check-input"
		type="radio"
		name="color-toggle"
		id="light-toggle"
		value="light"
	/>
	<label class="form-check-label" for="light-toggle"> Light Theme </label>
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
	<label class="form-check-label" for="dark-toggle"> Dark Theme </label>
</div>
<h5 class="mb-2 mt-2">Language</h5>
<div class="form-check">
	<input
		bind:group={lang}
		class="form-check-input"
		type="radio"
		name="lang-toggle"
		id="en-toggle"
		value="en"
	/>
	<label class="form-check-label" for="en-toggle"> English </label>
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
	<label class="form-check-label" for="ger-toggle"> German </label>
</div>
