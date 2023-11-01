<script lang="ts">
	import { _ } from "svelte-i18n";
	import { category, settingsGlobal, state } from "$lib/store";
	import Request from "../basic/Request.svelte";
	import { onMount } from "svelte";
	import Spinner from "../basic/Spinner.svelte";
	import EditCategories from "./EditCategories.svelte";

	let borrowing_duration = 0;
	let dnb_token = "";
	let mail_last_reminder = "";
	let mail_from = "";
	let mail_host = "";
	let mail_password = "";
	let mail_info_subject = "";
	let mail_info_content = "";
	let mail_overdue_subject = "";
	let mail_overdue_content = "";
	let mail_overdue2_subject = "";
	let mail_overdue2_content = "";

	let r: Request;
	let editDialog: EditCategories;

	async function update() {
		// get settings
		let data = await r.request("api/settings", "GET", null);

		if (data) {
			borrowing_duration = data.borrowing_duration ? data.borrowing_duration : 0;
			dnb_token = data.dnb_token ? data.dnb_token : "";
			mail_last_reminder = data.mail_last_reminder ? data.mail_last_reminder : "";
			mail_from = data.mail_from ? data.mail_from : "";
			mail_host = data.mail_host ? data.mail_host : "";
			mail_password = data.mail_password ? data.mail_password : "";
			mail_info_subject = data.mail_info_subject ? data.mail_info_subject : "";
			mail_info_content = data.mail_info_content ? data.mail_info_content : "";
			mail_overdue_subject = data.mail_overdue_subject ? data.mail_overdue_subject : "";
			mail_overdue_content = data.mail_overdue_content ? data.mail_overdue_content : "";
			mail_overdue2_subject = data.mail_overdue2_subject ? data.mail_overdue2_subject : "";
			mail_overdue2_content = data.mail_overdue2_content ? data.mail_overdue2_content : "";
		}

		settingsGlobal.set({
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
		});

		// get categories
		let data2 = await r.request("api/category", "GET", null);

		if (data2) category.set(data2);
	}

	async function updatePeriodically() {
		await update();
	}

	onMount(() => {
		// Run the `update` function immediately on mount
		updatePeriodically();

		const interval = setInterval(updatePeriodically, 300000);

		// Cleanup the interval when the component is unmounted
		return () => {
			clearInterval(interval);
		};
	});

	export async function save() {
		if (
			borrowing_duration !== $settingsGlobal.borrowing_duration ||
			dnb_token !== $settingsGlobal.dnb_token ||
			mail_last_reminder !== $settingsGlobal.mail_last_reminder ||
			mail_from !== $settingsGlobal.mail_from ||
			mail_host !== $settingsGlobal.mail_host ||
			mail_password !== $settingsGlobal.mail_password ||
			mail_info_subject !== $settingsGlobal.mail_info_subject ||
			mail_info_content !== $settingsGlobal.mail_info_content ||
			mail_overdue_subject !== $settingsGlobal.mail_overdue_subject ||
			mail_overdue_content !== $settingsGlobal.mail_overdue_content ||
			mail_overdue2_subject !== $settingsGlobal.mail_overdue2_subject ||
			mail_overdue2_content !== $settingsGlobal.mail_overdue2_content
		) {
			await r.request(
				"api/settings",
				"POST",
				JSON.stringify({
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
				})
			);

			settingsGlobal.set({
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
			});

			state.set({});
		}
	}

	export function cancel() {
		const settings = $settingsGlobal;

		borrowing_duration = settings.borrowing_duration;
		dnb_token = settings.dnb_token;
		mail_host = settings.mail_host;
		mail_from = settings.mail_from;
		mail_password = settings.mail_password;
		mail_info_subject = settings.mail_info_subject;
		mail_info_content = settings.mail_info_content;
		mail_overdue_subject = settings.mail_overdue_subject;
		mail_overdue_content = settings.mail_overdue_content;
		mail_overdue2_subject = settings.mail_overdue2_subject;
		mail_overdue2_content = settings.mail_overdue2_content;
	}

	let userResponse: Promise<any>;
	async function userUpdate() {
		await r.request("api/user-update-roles", "PATCH", null);
		state.set({});
	}
</script>

<Request bind:this={r} />

<EditCategories bind:this={editDialog} />

<h5 class="mb-2 mt-2">{$_(".pref.database.header")}</h5>
<div class="form">
	<button type="button" class="btn btn-secondary" on:click={() => editDialog.open()}
		>{$_(".category.edit")}</button
	>
</div>
<h5 class="mb-2 mt-2">{$_(".pref.borrowing.header")}</h5>
<div class="form">
	<label class="form-label" for="borrowing-time">{$_(".pref.borrowing.duration")}</label>
	<input bind:value={borrowing_duration} class="form-control" type="number" id="borrowing-time" />
</div>
<h5 class="mb-2 mt-2">{$_(".pref.user.header")}</h5>
<button
	type="button"
	class="btn btn-secondary"
	id="up"
	on:click={() => (userResponse = userUpdate())}
>
	<Spinner response={userResponse} />
	{$_(".pref.user.update")}</button
>
<h5 class="mb-2 mt-2">{$_(".pref.request.header")}</h5>
<div class="form">
	<label class="form-label" for="dnb">{$_(".pref.request.token")}</label>
	<input bind:value={dnb_token} class="form-control" type="text" id="dnb" />
</div>
<h5 class="mb-2 mt-2">{$_(".pref.mail.account.header")}</h5>
<div class="form">
	<div>
		<label class="form-label" for="host">{$_(".pref.mail.account.host")}</label>
		<input bind:value={mail_host} class="form-control" type="text" id="host" />
	</div>
	<div class="pt-1">
		<label class="form-label" for="sender">{$_(".pref.mail.account.from")}</label>
		<input bind:value={mail_from} class="form-control" type="text" id="sender" />
	</div>
	<div class="pt-1">
		<label class="form-label" for="password">{$_(".pref.mail.account.password")}</label>
		<input bind:value={mail_password} class="form-control" type="password" id="password" />
	</div>
</div>
<h5 class="mb-2 mt-2">{$_(".pref.mail.templates.header")}</h5>
<div class="form">
	<p>
		{$_(".mail.info")}
	</p>
	<ul class="nav nav-tabs card-header-pills mb-2 ms-1 me-1">
		<li class="nav-item" role="presentation">
			<button
				class="nav-link active"
				id="info-mail"
				data-bs-toggle="tab"
				data-bs-target="#info-mail-pane"
				type="button"
				role="tab"
				aria-controls="info-mail-pane"
				aria-selected="true">{$_(".mail.info.title")}</button
			>
		</li>
		<li class="nav-item" role="presentation">
			<button
				class="nav-link"
				id="warn-1-main"
				data-bs-toggle="tab"
				data-bs-target="#warn-1-main-pane"
				type="button"
				role="tab"
				aria-controls="warn-1-main-pane"
				aria-selected="false">{$_(".mail.overdue.title")}</button
			>
		</li>
		<li class="nav-item" role="presentation">
			<button
				class="nav-link"
				id="warn-2-main"
				data-bs-toggle="tab"
				data-bs-target="#warn-2-main-pane"
				type="button"
				role="tab"
				aria-controls="warn-2-main-pane"
				aria-selected="false">{$_(".mail.overdue2.title")}</button
			>
		</li>
	</ul>
	<div class="tab-content" id="tab-mail">
		<div
			class="tab-pane fade show active"
			id="info-mail-pane"
			role="tabpanel"
			aria-labelledby="info-mail"
			tabindex="0"
		>
			<div class="mb-3">
				<label for="infoSub" class="form-label">{$_(".mail.label.title")}</label>
				<input
					type="text"
					class="form-control"
					id="infoSub"
					placeholder={$_(".mail.label.title")}
					bind:value={mail_info_subject}
				/>
			</div>
			<div class="mb-3">
				<label for="infoCon" class="form-label">{$_(".mail.label.content")}</label>
				<textarea class="form-control" id="infoCon" rows="6" bind:value={mail_info_content} />
			</div>
		</div>
		<div
			class="tab-pane fade"
			id="warn-1-main-pane"
			role="tabpanel"
			aria-labelledby="warn-1-main"
			tabindex="0"
		>
			<div class="mb-3">
				<label for="overSub" class="form-label">{$_(".mail.label.title")}</label>
				<input
					type="text"
					class="form-control"
					id="overSub"
					placeholder={$_(".mail.label.title")}
					bind:value={mail_overdue_subject}
				/>
			</div>
			<div class="mb-3">
				<label for="overCon" class="form-label">{$_(".mail.label.content")}</label>
				<textarea class="form-control" id="overCon" rows="6" bind:value={mail_overdue_content} />
			</div>
		</div>
		<div
			class="tab-pane fade"
			id="warn-2-main-pane"
			role="tabpanel"
			aria-labelledby="warn-2-main"
			tabindex="0"
		>
			<div class="mb-3">
				<label for="overSub2" class="form-label">{$_(".mail.label.title")}</label>
				<input
					type="text"
					class="form-control"
					id="overSub2"
					placeholder={$_(".mail.label.title")}
					bind:value={mail_overdue2_subject}
				/>
			</div>
			<div class="mb-3">
				<label for="overCon2" class="form-label">{$_(".mail.label.content")}</label>
				<textarea class="form-control" id="overCon2" rows="6" bind:value={mail_overdue2_content} />
			</div>
		</div>
	</div>
</div>
