<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { Label } from '$lib/components/ui/label';
	import EditCategory from './EditCategory.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import { DateTime } from 'luxon';
	import { settingsGlobal, type GlobalSettings } from '$lib/store';
	import api from '$lib/api';
	import { areObjectsEqual, handle_result } from '$lib';
	import { Button } from '$lib/components/ui/button';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Separator } from '$lib/components/ui/separator';
	import DateInput from '$lib/components/ui/date-input/DateInput.svelte';

	let borrowing_duration = '0';
	let overdue_warning_delay = '0';
	let mail_last_reminder: DateTime = DateTime.fromISO('');
	let mail_from = '';
	let mail_host = '';
	let mail_password = '';

	let templates: Record<string, api.MailTemplate> = {
		info: { subject: '', body: '' },
		overdue: { subject: '', body: '' },
		overdue2: { subject: '', body: '' }
	};

	$: settings = {
		borrowing_duration: parseInt(borrowing_duration),
		overdue_warning_delay: parseInt(overdue_warning_delay),
		mail_last_reminder,
		mail_from,
		mail_host,
		mail_password,
		mail_info: templates.info,
		mail_overdue: templates.overdue,
		mail_overdue2: templates.overdue2
	};

	function set(s: GlobalSettings) {
		borrowing_duration = s.borrowing_duration.toString();
		overdue_warning_delay = s.overdue_warning_delay.toString();
		mail_last_reminder = s.mail_last_reminder;
		mail_from = s.mail_from;
		mail_host = s.mail_host;
		mail_password = s.mail_password;
		// update fields directly due to bindings
		templates.info.subject = s.mail_info.subject;
		templates.info.body = s.mail_info.body;
		templates.overdue.subject = s.mail_overdue.subject;
		templates.overdue.body = s.mail_overdue.body;
		templates.overdue2.subject = s.mail_overdue2.subject;
		templates.overdue2.body = s.mail_overdue2.body;
	}

	settingsGlobal.subscribe(set);

	let userResponse: Promise<void>;
	async function userUpdate() {
		await handle_result(api.user_update_roles());
	}

	let saveResponse: Promise<void>;
	async function save() {
		let data = {
			...settings,
			mail_last_reminder: settings?.mail_last_reminder.toISODate() ?? ''
		};

		handle_result(await api.settings_update(data));
		settingsGlobal.set(settings);
	}
</script>

<div class="space-y-4">
	<div class="space-y-10 p-4">
		<div>
			<h2 class="my-1.5">{$_('.category.edit')}</h2>
			<EditCategory />
		</div>
		<div>
			<h2 class="my-1.5">{$_('.pref.borrowing.duration')}</h2>
			<Input bind:value={borrowing_duration} type="number" />
		</div>
		<div>
			<h2 class="my-1.5">{$_('.pref.overdue.warning-delay')}</h2>
			<Input id="overdue-warning-delay" bind:value={overdue_warning_delay} type="number" />
		</div>
		<div>
			<h2 class="my-1.5">{$_('.pref.user.update')}</h2>
			<p class="my-1.5">{$_('.pref.user.update.info')}</p>
			<Button class="w-full" on:click={() => (userResponse = userUpdate())}>
				<Spinner response={userResponse} />
				{$_('.pref.user.update')}
			</Button>
		</div>
		<DateInput
			bind:date={mail_last_reminder}
			min={false}
			labelClass="text-md"
			label={$_('.pref.mail.last-reminder')}
		/>
		<div>
			<h2 class="my-1.5">{$_('.pref.cred')}</h2>
			<div class="space-y-2">
				<div>
					<Label for="host" class="my-1.5 block">{$_('.pref.mail.account.host')}</Label>
					<Input id="host" bind:value={mail_host} type="text" />
				</div>
				<div>
					<Label for="from" class="my-1.5 block">{$_('.pref.mail.account.from')}</Label>
					<Input id="from" bind:value={mail_from} type="text" />
				</div>
				<div>
					<Label for="password" class="my-1.5 block">{$_('.pref.mail.account.password')}</Label>
					<Input id="password" bind:value={mail_password} type="password" />
				</div>
			</div>
		</div>
		<div>
			<h2 class="my-1.5">{$_('.pref.mail.templates.header')}</h2>
			<p style="white-space: pre-line;">
				{$_('.mail.info')}
			</p>
			<Tabs.Root value="info" class="w-full">
				<Tabs.List class="grid w-full grid-cols-3">
					{#each Object.keys(templates) as name}
						<Tabs.Trigger value={name}>{$_(`.mail.${name}.title`)}</Tabs.Trigger>
					{/each}
				</Tabs.List>
				{#each Object.keys(templates) as name}
					<Tabs.Content value={name} class="space-y-2">
						<div>
							<Label for="title" class="my-1.5 block">{$_('.mail.label.title')}</Label>
							<Input id="title" bind:value={templates[name].subject} type="text" />
						</div>
						<div>
							<Label for="content" class="my-1.5 block">{$_('.mail.label.content')}</Label>
							<Textarea rows={6} id="content" bind:value={templates[name].body} />
						</div>
					</Tabs.Content>
				{/each}
			</Tabs.Root>
		</div>
	</div>
	<div class="bg-background sticky bottom-0 z-20 space-y-4 pb-4">
		<Separator />
		<div class="px-4">
			<Button
				class="w-full"
				disabled={areObjectsEqual(settings, $settingsGlobal)}
				on:click={() => (saveResponse = save())}
			>
				<Spinner response={saveResponse} />
				{$_('.action.apply')}
			</Button>
		</div>
	</div>
</div>
