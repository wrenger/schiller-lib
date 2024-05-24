<script lang="ts">
	import { _ } from 'svelte-i18n';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { setMode, mode } from 'mode-watcher';
	import { getLang, setLang } from '$lib/i18n';
	import Button from '$lib/components/ui/button/button.svelte';
	import Separator from '$lib/components/ui/separator/separator.svelte';
</script>

<div class="space-y-4 p-4">
	<div class="flex w-full flex-col gap-1.5">
		<Label class="text-md" for="theme">{$_('.pref.appearance.title')}</Label>
		<Select.Root
			selected={{ value: $mode, label: $_(`.pref.appearance.${$mode ? $mode : 'dark'}`) }}
		>
			<Select.Trigger class="w-full" id="theme">
				<Select.Value placeholder={$_('.pref.appearance.title')} />
			</Select.Trigger>
			<Select.Content>
				<Select.Item on:click={() => setMode('light')} value="light"
					>{$_('.pref.appearance.light')}</Select.Item
				>
				<Select.Item on:click={() => setMode('dark')} value="dark"
					>{$_('.pref.appearance.dark')}</Select.Item
				>
			</Select.Content>
		</Select.Root>
	</div>
	<Separator />
	<div class="flex w-full flex-col gap-1.5">
		<Label class="text-md" for="lang">{$_('.lang.title')}</Label>
		<Select.Root
			selected={{ value: getLang(), label: $_(`.lang.${getLang() ? getLang() : 'en'}`) }}
		>
			<Select.Trigger class="w-full" id="lang">
				<Select.Value placeholder={$_('.lang.title')} />
			</Select.Trigger>
			<Select.Content>
				<Select.Item on:click={() => setLang('en')} value="en">{$_('.lang.en')}</Select.Item>
				<Select.Item on:click={() => setLang('de')} value="de">{$_('.lang.de')}</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>
	<Separator />
	<div class="flex w-full flex-col gap-1.5">
		<Label class="text-md" for="logout">{$_('.info.session')}</Label>
		<Button id="logout" href="/auth/logout" variant="destructive">{$_('.action.logout')}</Button>
	</div>
</div>
