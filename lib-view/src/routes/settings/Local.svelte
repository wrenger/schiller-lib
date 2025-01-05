<script lang="ts">
	import { _, locales } from 'svelte-i18n';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { setMode, mode } from 'mode-watcher';
	import { getLang, setLang } from '$lib/i18n';
	import Button from '$lib/components/ui/button/button.svelte';

	const modes = ['light', 'dark'] as const;
</script>

<div class="space-y-10 p-4">
	<div>
		<Label class="text-md my-1.5 block" for="theme">{$_('.pref.appearance.title')}</Label>
		<Select.Root
			selected={{ value: $mode, label: $_(`.pref.appearance.${$mode ?? 'dark'}`) }}
		>
			<Select.Trigger class="w-full" id="theme">
				<Select.Value placeholder={$_('.pref.appearance.title')} />
			</Select.Trigger>
			<Select.Content>
				{#each modes as mode}
					<Select.Item on:click={() => setMode(mode)} value={mode}>
						{$_(`.pref.appearance.${mode}`)}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>
	<div>
		<Label class="text-md my-1.5 block" for="lang">{$_('.lang.title')}</Label>
		<Select.Root
			selected={{ value: getLang(), label: $_(`.lang.${getLang() ?? 'en'}`) }}
		>
			<Select.Trigger class="w-full" id="lang">
				<Select.Value placeholder={$_('.lang.title')} />
			</Select.Trigger>
			<Select.Content>
				{#each $locales as lang}
					<Select.Item on:click={() => setLang(lang)} value={lang}>
						{$_(`.lang.${lang}`)}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>
	<div>
		<Label class="text-md my-1.5 block" for="logout">{$_('.info.session')}</Label>
		<Button id="logout" class="w-full" href="/auth/logout" variant="destructive">
			{$_('.action.logout')}
		</Button>
	</div>
</div>
