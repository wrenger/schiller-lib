<script lang="ts">
	export let response: Promise<any>;
</script>

{#await response}
	<span
		id="book-confirm-button-spinner"
		class="spinner-border spinner-border-sm"
		role="status"
		aria-hidden="true"
	/>
{/await}
