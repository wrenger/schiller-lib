<script lang="ts">
	import { _, t } from "svelte-i18n";
	export let id: string = "";
	export let size: "default" | "medium" = "default";
	export let height: "default" | "fixed" = "default";
	export var onCancel: (() => void) | undefined = undefined;

	let dialog: HTMLDialogElement;
	let title: string | undefined;
	let text: string | undefined;

	export function open(title: string | undefined, text: string | undefined) {
		if (!dialog.attributes.getNamedItem("open")) {
			dialog.showModal();
		}
		title = title;
		text = text;
	}

	export function close() {
		dialog.close();
	}
</script>

<dialog
	{id}
	class="custom-dialog {size == 'medium' ? 'medium' : ''} {height == 'fixed' ? 'fixed' : ''}"
	bind:this={dialog}
	on:close
>
	<div class="card {height == 'fixed' ? 'fixed' : ''}">
		<div class="card-header">
			{#if title}
				{title}
			{/if}
			<slot name="header" />
		</div>
		<div class="card-body">
			{#if text}
				{text}
			{/if}
			<slot name="body" />
		</div>
		<div class="card-footer d-flex justify-content-between">
			<button
				type="button"
				class="btn btn-secondary"
				on:click={() => {
					dialog.close();
					if (onCancel) onCancel();
				}}>{$_(".action.close")}</button
			>
			<slot name="footer" />
		</div>
	</div>
</dialog>

<style>
	.custom-dialog {
		padding: 0px;
		height: fit-content;
		width: 30rem;
		border: none;
		background: none;
		position: relative;
	}
	.custom-dialog.fixed {
		height: 100%;
	}
	.custom-dialog.medium {
		width: 50rem;
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.4);
	}
	.card.fixed {
		height: 100%;
	}
	.card-body {
		overflow-y: auto;
		display: block;
		min-height: 200px;
	}
</style>
