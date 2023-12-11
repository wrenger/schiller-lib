<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { getLang, setLang } from '$lib/i18n';
	import { LightSwitch } from '@skeletonlabs/skeleton';

	let lang = getLang();

	$: setLang(lang || undefined);
</script>

<h5 class="h5">{$_('.pref.appearance.title')}</h5>
<LightSwitch />
<h5 class="h5">{$_('.lang.title')}</h5>
<div class="space-y-2">
	<label class="flex items-center space-x-2">
		<input class="radio" type="radio" checked name="radio-direct" value="en" bind:group={lang} />
		<p>{$_('.lang.en')}</p>
	</label>
	<label class="flex items-center space-x-2">
		<input class="radio" type="radio" name="radio-direct" value="de" bind:group={lang} />
		<p>{$_('.lang.de')}</p>
	</label>
</div>
