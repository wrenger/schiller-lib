<script lang="ts">
	import { _ } from "svelte-i18n";
	let dialog: HTMLDialogElement;

	export let size: "default" | "medium" = "default";
	export let height: "default" | "fixed" = "default";
	export var onCancel: (() => void) | undefined;

	export function open() {
		if (!dialog.attributes.getNamedItem("open")) {
			dialog.showModal();
		}
	}
</script>

<dialog
	class="custom-dialog {size == 'medium' ? 'medium' : ''} {height == 'fixed' ? 'fixed' : ''}"
	bind:this={dialog}
	on:close
>
	<div class="card {height == 'fixed' ? 'fixed' : ''}">
		<div class="card-header">
			<slot name="header" />
		</div>
		<div class="card-body">
			<slot name="body" />
		</div>
		<div class="card-footer">
			<slot name="footer" />
			<button
				type="button"
				class="btn btn-secondary"
				on:click={() => {
					dialog.close();
					if (onCancel) onCancel();
				}}>{$_(".action.close")}</button
			>
		</div>
	</div>
</dialog>

<style>
	.custom-dialog {
		padding: 0px;
		height: fit-content;
		width: 20rem;
		border: none;
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
	}
</style>
