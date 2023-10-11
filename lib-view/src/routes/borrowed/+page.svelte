<script lang="ts">
	import { _ } from "svelte-i18n";
</script>

<svelte:head>
	<title>{$_(".book.borrowed")}</title>
	<meta name="description" content={$_(".book.borrowed")} />
</svelte:head>

<p>({$_(".book.borrowed")})</p>
