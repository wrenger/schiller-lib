<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { settingsGlobal, type GlobalSettings, state } from '$lib/store';
	import api from '../../lib/api';
	import { Accordion, AccordionItem, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import DateField from '../basic/DateField.svelte';
	import Spinner from '../basic/Spinner.svelte';
	import { DateTime } from 'luxon';
	import EditCategories from './EditCategories.svelte';

	let borrowing_duration = 0;
	let dnb_token = '';
	let mail_last_reminder: DateTime = DateTime.fromISO('');
	let mail_from = '';
	let mail_host = '';
	let mail_password = '';
	let mail_info_subject = '';
	let mail_info_content = '';
	let mail_overdue_subject = '';
	let mail_overdue_content = '';
	let mail_overdue2_subject = '';
	let mail_overdue2_content = '';

	export function get(): GlobalSettings {
		return {
			borrowing_duration,
			dnb_token,
			mail_last_reminder,
			mail_from,
			mail_host,
			mail_password,
			mail_info_subject,
			mail_info_content,
			mail_overdue_subject,
			mail_overdue_content,
			mail_overdue2_subject,
			mail_overdue2_content
		};
	}

	export function set(s: GlobalSettings) {
		dnb_token = s.dnb_token;
		borrowing_duration = s.borrowing_duration;
		mail_last_reminder = s.mail_last_reminder;
		mail_from = s.mail_from;
		mail_host = s.mail_host;
		mail_password = s.mail_password;
		mail_info_subject = s.mail_info_subject;
		mail_info_content = s.mail_info_content;
		mail_overdue_subject = s.mail_overdue_subject;
		mail_overdue_content = s.mail_overdue_content;
		mail_overdue2_subject = s.mail_overdue2_subject;
		mail_overdue2_content = s.mail_overdue2_content;
	}

	settingsGlobal.subscribe(set);

	let tab = 0;

	let userResponse: Promise<void>;
	async function userUpdate() {
		await api.user_update_roles();
		state.set({});
	}
</script>

<Accordion>
	<AccordionItem>
		<svelte:fragment slot="lead"><i class="fa-solid fa-layer-group"></i></svelte:fragment>
		<svelte:fragment slot="summary">{$_('.category.edit')}</svelte:fragment>
		<svelte:fragment slot="content"><EditCategories /></svelte:fragment>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="lead"><i class="fa-solid fa-clock-rotate-left"></i></svelte:fragment>
		<svelte:fragment slot="summary">{$_('.pref.borrowing.duration')}</svelte:fragment>
		<svelte:fragment slot="content">
			<label class="label">
				<span>{$_('.pref.borrowing.duration')}</span>
				<input
					bind:value={borrowing_duration}
					class="input"
					type="number"
					placeholder={$_('.pref.borrowing.duration')}
				/>
			</label>
		</svelte:fragment>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="lead"><i class="fa-solid fa-user"></i></svelte:fragment>
		<svelte:fragment slot="summary">{$_('.pref.user.update')}</svelte:fragment>
		<svelte:fragment slot="content">
			<button
				type="button"
				class="btn variant-filled"
				id="up"
				on:click={() => (userResponse = userUpdate())}
				><Spinner response={userResponse} />
				{$_('.pref.user.update')}</button
			>
		</svelte:fragment>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="lead"><i class="fa-solid fa-database"></i></svelte:fragment>
		<svelte:fragment slot="summary">{$_('.pref.request.token')}</svelte:fragment>
		<svelte:fragment slot="content">
			<label class="label">
				<span>{$_('.pref.request.token')}</span>
				<input
					bind:value={dnb_token}
					class="input"
					type="text"
					placeholder={$_('.pref.request.token')}
				/>
			</label>
		</svelte:fragment>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="lead"><i class="fa-solid fa-key"></i></svelte:fragment>
		<svelte:fragment slot="summary">{$_('.pref.cred')}</svelte:fragment>
		<svelte:fragment slot="content">
			<label class="label">
				<span>{$_('.pref.mail.account.host')}</span>
				<input
					bind:value={mail_host}
					class="input"
					type="text"
					placeholder={$_('.pref.mail.account.host')}
				/>
			</label>
			<label class="label">
				<span>{$_('.pref.mail.account.from')}</span>
				<input
					bind:value={mail_from}
					class="input"
					type="text"
					placeholder={$_('.pref.mail.account.from')}
				/>
			</label>
			<label class="label">
				<span>{$_('.pref.mail.account.password')}</span>
				<input
					bind:value={mail_password}
					class="input"
					type="password"
					placeholder={$_('.pref.mail.account.password')}
				/>
			</label>

			<DateField
				bind:date={mail_last_reminder}
				min={false}
				label={$_('.pref.mail.last-reminder')}
			/>
		</svelte:fragment>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="lead"><i class="fa-solid fa-envelope"></i></svelte:fragment>
		<svelte:fragment slot="summary">{$_('.pref.mail.templates.header')}</svelte:fragment>
		<svelte:fragment slot="content">
			<div class="form">
				<p>
					{$_('.mail.info')}
				</p>
				<div class="relative">
					<TabGroup class="max-w-5xl mx-auto hide-scrollbar" style="outline: none;">
						<Tab bind:group={tab} name="tab1" value={0}>{$_('.mail.info.title')}</Tab>
						<Tab bind:group={tab} name="tab2" value={1}>{$_('.mail.overdue.title')}</Tab>
						<Tab bind:group={tab} name="tab3" value={2}>{$_('.mail.overdue2.title')}</Tab>
					</TabGroup>

					<div id="panels" class="pt-4">
						{#if tab === 0}
							<label class="label">
								<span>{$_('.mail.label.title')}</span>
								<input
									class="input"
									type="text"
									placeholder={$_('.mail.label.title')}
									bind:value={mail_info_subject}
								/>
							</label>

							<label class="label">
								<span>{$_('.mail.label.content')}</span>
								<textarea
									class="textarea"
									rows="6"
									placeholder={$_('.mail.label.content')}
									bind:value={mail_info_content}
								/>
							</label>
						{:else if tab === 1}
							<label class="label">
								<span>{$_('.mail.label.title')}</span>
								<input
									class="input"
									type="text"
									placeholder={$_('.mail.label.title')}
									bind:value={mail_overdue_subject}
								/>
							</label>

							<label class="label">
								<span>{$_('.mail.label.content')}</span>
								<textarea
									class="textarea"
									rows="6"
									placeholder={$_('.mail.label.content')}
									bind:value={mail_overdue_content}
								/>
							</label>
						{:else if tab === 2}
							<label class="label">
								<span>{$_('.mail.label.title')}</span>
								<input
									class="input"
									type="text"
									placeholder={$_('.mail.label.title')}
									bind:value={mail_overdue2_subject}
								/>
							</label>

							<label class="label">
								<span>{$_('.mail.label.content')}</span>
								<textarea
									class="textarea"
									rows="6"
									placeholder={$_('.mail.label.content')}
									bind:value={mail_overdue2_content}
								/>
							</label>
						{/if}
					</div>
				</div>
			</div>
		</svelte:fragment>
	</AccordionItem>
</Accordion>
