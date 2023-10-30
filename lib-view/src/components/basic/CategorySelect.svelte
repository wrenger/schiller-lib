<script lang="ts">
	import { _ } from "svelte-i18n";
	import Request from "../../components/basic/Request.svelte";
	import { category } from "$lib/store";
	export let value: string | null = "";
	export let label: string | undefined = undefined;
	export let disabled = false;

	let items = $category;
	$: items = $category;
	let r: Request;
</script>

<Request bind:this={r} />
{#if label}
	<label for="select-{label}" class="form-label">{label}</label>
{/if}
<select class="form-select" id={label ? `select-${label}` : ""} bind:value {disabled}>
	<option selected value={null}>{$_(".action.select")}</option>
	{#each items as item}
		<option value={item.id}>{item.id}</option>
	{/each}
</select>
