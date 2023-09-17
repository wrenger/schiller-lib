<script lang="ts">
	let borrowing_time = 28;
	let separator = "|";
	let dnb = "ouehfuseifushiuefbiyagqyiiywfqgefybigg";
	let host = "whatever.com";
	let sender = "idk@whatever.com";
	let password = "1234";
	let title1 = "";
	let text1 = "";
	let title2 = "";
	let text2 = "";
	let title3 = "";
	let text3 = "";

	let initialSettings = {
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
	};

	export function save() {
		// todo: Server request
		initialSettings = {
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
		};
	}

	export function cancel() {
		borrowing_time = initialSettings.borrowing_time;
		separator = initialSettings.separator;
		dnb = initialSettings.dnb;
		host = initialSettings.host;
		sender = initialSettings.sender;
		password = initialSettings.password;
		title1 = initialSettings.title1;
		text1 = initialSettings.text1;
		title2 = initialSettings.title2;
		text2 = initialSettings.text2;
		title3 = initialSettings.title3;
		text3 = initialSettings.text3;
	}
</script>

<h5 class="mb-2 mt-2">Database</h5>
<div class="form">
	<button type="button" class="btn btn-secondary" on:click={() => console.log("Change Categories")}
		>Change Categories</button
	>
</div>
<h5 class="mb-2 mt-2">Borrowing</h5>
<div class="form">
	<label class="form-label" for="borrowing-time">Time (Days):</label>
	<input bind:value={borrowing_time} class="form-control" type="number" id="borrowing-time" />
</div>
<h5 class="mb-2 mt-2">Users</h5>
<div class="form">
	<label class="form-label" for="separator">Separator:</label>
	<input bind:value={separator} class="form-control" type="text" id="separator" />
	<div class="pt-1">
		<div class="row">
			<div class="col">
				<label class="form-label" for="file-upload">Upload User File:</label>
				<input type="file" class="form-control" id="file-upload" />
			</div>
			<div class="col">
				<label class="form-label d-flex" for="up">Update User Data:</label>
				<button
					type="button"
					class="btn btn-secondary"
					id="up"
					on:click={() => console.log("Update User Data")}>Update User Data</button
				>
			</div>
		</div>
	</div>
</div>
<h5 class="mb-2 mt-2">Media</h5>
<div class="form">
	<label class="form-label" for="dnb">DNB Token:</label>
	<input bind:value={dnb} class="form-control" type="text" id="dnb" />
</div>
<h5 class="mb-2 mt-2">E-Mail Account</h5>
<div class="form">
	<div>
		<label class="form-label" for="host">Host:</label>
		<input bind:value={host} class="form-control" type="text" id="host" />
	</div>
	<div class="pt-1">
		<label class="form-label" for="sender">Sender:</label>
		<input bind:value={sender} class="form-control" type="text" id="sender" />
	</div>
	<div class="pt-1">
		<label class="form-label" for="password">Password:</label>
		<input bind:value={password} class="form-control" type="password" id="password" />
	</div>
</div>
<h5 class="mb-2 mt-2">E-Mail Template</h5>
<div class="form">
	<p>
		You can also use keywords for certain values: "{"{username}"}": Username, "{"{booktitle}"}":
		Book Title
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
				aria-selected="true">Info</button
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
				aria-selected="false">Warn 1</button
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
				aria-selected="false">Warn 2</button
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
				<label for="Title1" class="form-label">Title</label>
				<input
					type="text"
					class="form-control"
					id="Title1"
					placeholder="Title"
					bind:value={title1}
				/>
			</div>
			<div class="mb-3">
				<label for="Text1" class="form-label">Text</label>
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
				<label for="Title2" class="form-label">Title</label>
				<input
					type="text"
					class="form-control"
					id="Title2"
					placeholder="Title"
					bind:value={title2}
				/>
			</div>
			<div class="mb-3">
				<label for="Text2" class="form-label">Text</label>
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
				<label for="Title3" class="form-label">Title</label>
				<input
					type="text"
					class="form-control"
					id="Title3"
					placeholder="Title"
					bind:value={title3}
				/>
			</div>
			<div class="mb-3">
				<label for="Text3" class="form-label">Text</label>
				<textarea class="form-control" id="Text3" rows="6" bind:value={text3} />
			</div>
		</div>
	</div>
</div>
