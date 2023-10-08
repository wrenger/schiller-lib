<script lang="ts">
	import { _ } from "svelte-i18n";
</script>

<svelte:head>
	<title>{$_(".alert.info")}</title>
	<meta name="description" content={$_(".alert.info")} />
</svelte:head>

<p>({$_(".alert.info")})</p>
