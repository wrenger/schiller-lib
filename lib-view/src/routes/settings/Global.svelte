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
	let dnb_token = '';
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
		dnb_token,
		mail_last_reminder,
		mail_from,
		mail_host,
		mail_password,
		mail_info: templates.info,
		mail_overdue: templates.overdue,
		mail_overdue2: templates.overdue2
	};

	function set(s: GlobalSettings) {
		dnb_token = s.dnb_token;
		borrowing_duration = s.borrowing_duration.toString();
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

<div class="space-y-4 p-3 pb-0">
	<div class="space-y-4 p-1">
		<div class="flex w-full flex-col gap-1.5">
			<Label class="text-md" for="category">{$_('.category.edit')}</Label>
			<EditCategory />
		</div>
		<Separator />
		<div class="flex w-full flex-col gap-1.5">
			<Label class="text-md" for="borrowing-duration">{$_('.pref.borrowing.duration')}</Label>
			<Input
				id="borrowing-duration"
				bind:value={borrowing_duration}
				type="number"
				placeholder={$_('.pref.borrowing.duration')}
			/>
		</div>
		<Separator />
		<div class="flex w-full flex-col gap-1.5">
			<Label class="text-md" for="user-update">{$_('.pref.user.update')}</Label>
			<Button id="user-update" on:click={() => (userResponse = userUpdate())}>
				<Spinner response={userResponse} />
				{$_('.pref.user.update')}
			</Button>
		</div>
		<Separator />
		<div class="flex w-full flex-col gap-1.5">
			<Label class="text-md" for="dnb-token">{$_('.pref.request.token')}</Label>
			<Input
				id="dnb-token"
				bind:value={dnb_token}
				type="text"
				placeholder={$_('.pref.request.token')}
			/>
		</div>
		<Separator />
		<DateInput
			bind:date={mail_last_reminder}
			min={false}
			labelClass="text-md"
			label={$_('.pref.mail.last-reminder')}
		/>
		<Separator />
		<div class="flex w-full flex-col gap-1.5">
			<Label class="text-md">{$_('.pref.cred')}</Label>
			<div class="space-y-2">
				<div class="flex w-full flex-col gap-1.5">
					<Label for="hostname">{$_('.pref.mail.account.host')}</Label>
					<Input
						id="hostname"
						bind:value={mail_host}
						type="text"
						placeholder={$_('.pref.mail.account.host')}
					/>
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="from">{$_('.pref.mail.account.from')}</Label>
					<Input
						id="from"
						bind:value={mail_from}
						type="text"
						placeholder={$_('.pref.mail.account.from')}
					/>
				</div>
				<div class="flex w-full flex-col gap-1.5">
					<Label for="password">{$_('.pref.mail.account.password')}</Label>
					<Input
						id="password"
						bind:value={mail_password}
						type="password"
						placeholder={$_('.pref.mail.account.password')}
					/>
				</div>
			</div>
		</div>
		<Separator />
		<div class="flex w-full flex-col gap-1.5">
			<Label class="text-md">{$_('.pref.mail.templates.header')}</Label>
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
					<Tabs.Content value={name}>
						<div class="space-y-2">
							<div class="flex w-full flex-col gap-1.5">
								<Label for="title">{$_('.mail.label.title')}</Label>
								<Input
									id="title"
									bind:value={templates[name].subject}
									type="text"
									placeholder={$_('.mail.label.title')}
								/>
							</div>
							<div class="flex w-full flex-col gap-1.5">
								<Label for="content">{$_('.mail.label.content')}</Label>
								<Textarea
									rows={6}
									id="content"
									bind:value={templates[name].body}
									placeholder={$_('.mail.label.content')}
								/>
							</div>
						</div>
					</Tabs.Content>
				{/each}
			</Tabs.Root>
		</div>
	</div>
	<div class="sticky bottom-0 z-20 space-y-4 bg-background p-1 pb-4 pt-0">
		<Separator />
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
