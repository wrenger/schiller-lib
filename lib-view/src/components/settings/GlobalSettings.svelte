<script lang="ts">
	import { _ } from "svelte-i18n";
	import { settingsLocal } from "$lib/store";

	// todo: initial server request
	let borrowing_time = 28;
	let separator = "|";
	let dnb = "ouehfuseifushiuefbiyagqyiiywfqgefybigg";
	let host = "whatever.com";
	let sender = "idk@whatever.com";
	let password = "1234";
	let title1 = $_(".mail.info.subject", { locale: "de" });
	let text1 = $_(".mail.info.content", { locale: "de" });
	let title2 = $_(".mail.overdue.subject", { locale: "de" });
	let text2 = $_(".mail.overdue.content", { locale: "de" });
	let title3 = $_(".mail.overdue2.subject", { locale: "de" });
	let text3 = $_(".mail.overdue2.content", { locale: "de" });

	save();

	export function save() {
		// todo: Server request
		settingsLocal.set({
			borrowing_time,
			separator,
			dnb,
			host,
			sender,
			password,
			title1,
			text1,
			title2,
			text2,
			title3,
			text3
		});
	}

	export function cancel() {
		const settings = $settingsLocal;

		borrowing_time = settings.borrowing_time;
		separator = settings.separator;
		dnb = settings.dnb;
		host = settings.host;
		sender = settings.sender;
		password = settings.password;
		title1 = settings.title1;
		text1 = settings.text1;
		title2 = settings.title2;
		text2 = settings.text2;
		title3 = settings.title3;
		text3 = settings.text3;
	}
</script>

<h5 class="mb-2 mt-2">{$_(".pref.database.header")}</h5>
<div class="form">
	<button type="button" class="btn btn-secondary" on:click={() => console.log("Change Categories")}
		>{$_(".category.edit")}</button
	>
</div>
<h5 class="mb-2 mt-2">{$_(".pref.borrowing.header")}</h5>
<div class="form">
	<label class="form-label" for="borrowing-time">{$_(".pref.borrowing.duration")}</label>
	<input bind:value={borrowing_time} class="form-control" type="number" id="borrowing-time" />
</div>
<h5 class="mb-2 mt-2">{$_(".pref.user.header")}</h5>
<div class="form">
	<label class="form-label" for="separator">{$_(".pref.user.delimiter")}</label>
	<input bind:value={separator} class="form-control" type="text" id="separator" />
	<div class="pt-1">
		<div class="row">
			<div class="col">
				<label class="form-label" for="file-upload">{$_(".pref.user.path")}</label>
				<input type="file" class="form-control" id="file-upload" />
			</div>
			<div class="col">
				<label class="form-label d-flex" for="up">{$_(".pref.user.update")}</label>
				<button
					type="button"
					class="btn btn-secondary"
					id="up"
					on:click={() => console.log("Updated User Data")}>{$_(".pref.user.update")}</button
				>
			</div>
		</div>
	</div>
</div>
<h5 class="mb-2 mt-2">{$_(".pref.request.header")}</h5>
<div class="form">
	<label class="form-label" for="dnb">{$_(".pref.request.token")}</label>
	<input bind:value={dnb} class="form-control" type="text" id="dnb" />
</div>
<h5 class="mb-2 mt-2">{$_(".pref.mail.account.header")}</h5>
<div class="form">
	<div>
		<label class="form-label" for="host">{$_(".pref.mail.account.host")}</label>
		<input bind:value={host} class="form-control" type="text" id="host" />
	</div>
	<div class="pt-1">
		<label class="form-label" for="sender">{$_(".pref.mail.account.from")}</label>
		<input bind:value={sender} class="form-control" type="text" id="sender" />
	</div>
	<div class="pt-1">
		<label class="form-label" for="password">{$_(".pref.mail.account.password")}</label>
		<input bind:value={password} class="form-control" type="password" id="password" />
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
				<label for="Title1" class="form-label">{$_(".mail.label.title")}</label>
				<input
					type="text"
					class="form-control"
					id="Title1"
					placeholder={$_(".mail.label.title")}
					bind:value={title1}
				/>
			</div>
			<div class="mb-3">
				<label for="Text1" class="form-label">{$_(".mail.label.content")}</label>
				<textarea class="form-control" id="Text1" rows="6" bind:value={text1} />
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
				<label for="Title2" class="form-label">{$_(".mail.label.title")}</label>
				<input
					type="text"
					class="form-control"
					id="Title2"
					placeholder="{$_('.mail.label.title')}<"
					bind:value={title2}
				/>
			</div>
			<div class="mb-3">
				<label for="Text2" class="form-label">{$_(".mail.label.content")}</label>
				<textarea class="form-control" id="Text2" rows="6" bind:value={text2} />
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
				<label for="Title3" class="form-label">{$_(".mail.label.title")}</label>
				<input
					type="text"
					class="form-control"
					id="Title3"
					placeholder="{$_('.mail.label.title')}<"
					bind:value={title3}
				/>
			</div>
			<div class="mb-3">
				<label for="Text3" class="form-label">{$_(".mail.label.content")}</label>
				<textarea class="form-control" id="Text3" rows="6" bind:value={text3} />
			</div>
		</div>
	</div>
</div>
