<script lang="ts">
	import { _ } from "svelte-i18n";
</script>

<svelte:head>
	<title>{$_(".alert.overdue")}</title>
	<meta name="description" content={$_(".alert.overdue")} />
</svelte:head>

<p>({$_(".alert.overdue")})</p>
