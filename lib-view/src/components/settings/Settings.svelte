<script lang="ts">
	import { _ } from "svelte-i18n";
	import Dialog from "../basic/Dialog.svelte";
	import GlobalSettings from "./GlobalSettings.svelte";
	import LocalSettings from "./LocalSettings.svelte";
	import Spinner from "../basic/Spinner.svelte";
	let dialog: Dialog;
	let local: LocalSettings;
	let global: GlobalSettings;

	function onCancel() {
		if (local) local.cancel();
		if (global) global.cancel();
	}

	let safeResponse: Promise<any>;
	async function onSave() {
		if (local) local.save();
		if (global) await global.save();
	}
</script>

<div class="d-flex">
	<button
		class="btn btn-dark"
		type="button"
		title={$_(".pref.title")}
		on:click={() => dialog.open()}
	>
		<i class="bi bi-gear" />
	</button>
	<Dialog bind:this={dialog} {onCancel} size="medium" height="fixed">
		<span slot="header">
			<ul class="nav nav-tabs card-header-tabs">
				<li class="nav-item" role="presentation">
					<button
						class="nav-link active"
						id="local-tab"
						data-bs-toggle="tab"
						data-bs-target="#local-tab-pane"
						type="button"
						role="tab"
						aria-controls="local-tab-pane"
						aria-selected="true">{$_(".pref.local")}</button
					>
				</li>
				<li class="nav-item" role="presentation">
					<button
						class="nav-link"
						id="global-tab"
						data-bs-toggle="tab"
						data-bs-target="#global-tab-pane"
						type="button"
						role="tab"
						aria-controls="global-tab-pane"
						aria-selected="false">{$_(".pref.global")}</button
					>
				</li>
			</ul>
		</span>
		<span slot="body">
			<div class="tab-content" id="myTabContent">
				<div
					class="tab-pane fade show active"
					id="local-tab-pane"
					role="tabpanel"
					aria-labelledby="local-tab"
					tabindex="0"
				>
					<LocalSettings bind:this={local} />
				</div>
				<div
					class="tab-pane fade"
					id="global-tab-pane"
					role="tabpanel"
					aria-labelledby="global-tab"
					tabindex="0"
				>
					<GlobalSettings bind:this={global} />
				</div>
			</div>
		</span>
		<button
			type="button"
			class="btn btn-primary"
			slot="footer"
			on:click={() => (safeResponse = onSave())}
		>
			<Spinner response={safeResponse} />
			{$_(".action.apply")}</button
		>
	</Dialog>
</div>
