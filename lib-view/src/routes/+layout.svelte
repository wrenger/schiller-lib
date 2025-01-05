<script lang="ts">
	import '../app.pcss';
	import { _ } from 'svelte-i18n';
	import { ModeWatcher, mode } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner';
	import { Button } from '$lib/components/ui/button';
	import { BookMarked, CalendarClock, Info, Settings, User } from 'lucide-svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import * as Avatar from '$lib/components/ui/avatar';
	import { page } from '$app/stores';
	import { Separator } from '$lib/components/ui/separator';
	import { categories, stats, settingsGlobal } from '$lib/store';
	import api from '$lib/api';
	import { DateTime } from 'luxon';
	import { onMount } from 'svelte';
	import Reminder from './Reminder.svelte';
	import { handle_result } from '$lib';

	// Getting needed infos
	async function update() {
		// Get settings
		let settings_data = handle_result(await api.settings_get());
		settingsGlobal.set({
			...settings_data,
			mail_last_reminder: DateTime.fromISO(settings_data.mail_last_reminder)
		});
		categories.set(handle_result(await api.category_list()));
		stats.set(handle_result(await api.stats()));
	}

	// Update periodically after and on Mount
	onMount(() => {
		update();
		const interval = setInterval(update, 300_000);
		return () => clearInterval(interval);
	});
</script>

<Toaster id="toaster" theme={$mode} class={'z-[100]'} />

<ModeWatcher disableTransitions={false} />

<Reminder />

<div class="grid h-full grid-cols-[60px_auto]">
	<div class="grid h-full w-full grid-rows-[59px_1px_auto] border-r">
		<div class="flex items-center justify-center">
			<a href={$page.url.pathname} data-sveltekit-reload>
				<Avatar.Root class="rounded-none">
					<Avatar.Image src="/favicon.png" alt="schiller-lib" />
					<Avatar.Fallback>SL</Avatar.Fallback>
				</Avatar.Root>
			</a>
		</div>
		<Separator />
		<div class="grid grid-rows-[auto_1px_59px]">
			<div class="flex flex-col items-center gap-1 pb-2 pt-2">
				<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
					<Tooltip.Trigger asChild let:builder>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg {$page.url.pathname == '/books' ? 'border' : ''}"
							aria-label={$_('.search.book')}
							builders={[builder]}
							href="/books"
						>
							<BookMarked class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="right" sideOffset={5}>
						{$_('.search.book')} <span class="text-muted-foreground">{$stats.books}</span>
					</Tooltip.Content>
				</Tooltip.Root>
				<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
					<Tooltip.Trigger asChild let:builder>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg {$page.url.pathname == '/users' ? 'border' : ''}"
							aria-label={$_('.search.user')}
							builders={[builder]}
							href="/users"
						>
							<User class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="right" sideOffset={5}>
						{$_('.search.user')} <span class="text-muted-foreground">{$stats.users}</span>
					</Tooltip.Content>
				</Tooltip.Root>
				<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
					<Tooltip.Trigger asChild let:builder>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg {$page.url.pathname == '/overdues' ? 'border' : ''}"
							aria-label={$_('.book.overdues')}
							builders={[builder]}
							href="/overdues"
						>
							<CalendarClock class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="right" sideOffset={5}>
						{$_('.book.overdues')} <span class="text-muted-foreground">{$stats.overdues}</span>
					</Tooltip.Content>
				</Tooltip.Root>
				<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
					<Tooltip.Trigger asChild let:builder>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg {$page.url.pathname == '/info' ? 'border' : ''}"
							aria-label={$_('.alert.info')}
							builders={[builder]}
							href="/info"
						>
							<Info class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="right" sideOffset={5}>{$_('.alert.info')}</Tooltip.Content>
				</Tooltip.Root>
			</div>
			<Separator />
			<div class="flex flex-col items-center justify-center gap-1">
				<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
					<Tooltip.Trigger asChild let:builder>
						<Button
							variant="ghost"
							size="icon"
							class="rounded-lg {$page.url.pathname == '/settings' ? 'border' : ''}"
							aria-label={$_('.pref.title')}
							builders={[builder]}
							href="/settings"
						>
							<Settings class="size-5" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content side="right" sideOffset={5}>{$_('.pref.title')}</Tooltip.Content>
				</Tooltip.Root>
			</div>
		</div>
	</div>

	<slot />
</div>
