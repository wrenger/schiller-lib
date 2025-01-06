<script lang="ts">
	import '../app.pcss';
	import { _ } from 'svelte-i18n';
	import { ModeWatcher, mode } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner';
	import { BookMarked, CalendarClock, Info, Settings, User } from 'lucide-svelte';
	import * as Avatar from '$lib/components/ui/avatar';
	import { page } from '$app/stores';
	import { Separator } from '$lib/components/ui/separator';
	import { categories, stats, settingsGlobal, reminderDialog } from '$lib/store';
	import api from '$lib/api';
	import { DateTime } from 'luxon';
	import { onMount } from 'svelte';
	import Reminder from './Reminder.svelte';
	import { handle_result } from '$lib';
	import IconButton from '$lib/components/custom/IconButton.svelte';

	let reminder: Reminder;
	$: reminderDialog.set(reminder);

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

<Reminder bind:this={reminder} />

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
			<div class="flex flex-col items-center gap-1 py-3">
				<IconButton
					icon={BookMarked}
					label={$_('.search.book')}
					extra={$stats.books + ''}
					variant={$page.url.pathname == '/books' ? 'outline' : 'ghost'}
					href="/books"
					tooltip_side="right"
				/>
				<IconButton
					icon={User}
					label={$_('.search.user')}
					extra={$stats.users + ''}
					variant={$page.url.pathname == '/users' ? 'outline' : 'ghost'}
					href="/users"
					tooltip_side="right"
				/>
				<IconButton
					icon={CalendarClock}
					label={$_('.book.overdues')}
					extra={$stats.overdues + ''}
					variant={$page.url.pathname == '/overdues' ? 'outline' : 'ghost'}
					href="/overdues"
					tooltip_side="right"
				/>
				<IconButton
					icon={Info}
					label={$_('.alert.info')}
					variant={$page.url.pathname == '/info' ? 'outline' : 'ghost'}
					href="/info"
					tooltip_side="right"
				/>
			</div>
			<Separator />
			<div class="flex flex-col items-center justify-center gap-1">
				<IconButton
					icon={Settings}
					label={$_('.pref.title')}
					variant={$page.url.pathname == '/settings' ? 'outline' : 'ghost'}
					href="/settings"
					tooltip_side="right"
				/>
			</div>
		</div>
	</div>

	<slot />
</div>
