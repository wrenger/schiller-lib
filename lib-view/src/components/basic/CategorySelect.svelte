<script lang="ts">
	import { _ } from "svelte-i18n";
	import { category } from "$lib/store";

	export let value: string | null = "";
	export let label: string | undefined = undefined;
	export let disabled = false;
	export var onChange: (() => void) | undefined = undefined;

	let items = $category;
	$: items = $category;
</script>

{#if label}
	<label for="select-{label}" class="form-label">{label}</label>
{/if}
<select
	class="form-select"
	id={label ? `select-${label}` : ""}
	bind:value
	{disabled}
	on:change={onChange}
>
	<option selected value={null}>{$_(".action.select")}</option>
	<option value={""}>{$_(".category.none")}</option>
	{#each items as item}
		<option value={item.id}>{item.id} - {item.name} - {item.section}</option>
	{/each}
</select>
