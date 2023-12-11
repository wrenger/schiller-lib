<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { category } from '$lib/store';

	export let value: string = '';
	export let label: string | undefined = undefined;
	export let disabled = false;
	export var onChange: (() => void) | undefined = undefined;

	let items = $category;
	$: items = $category;
</script>

<label class="label">
	{#if label}
		<span>{label}</span>
	{/if}
	<select
		class="select"
		id={label ? `select-${label}` : undefined}
		bind:value
		{disabled}
		on:change={onChange}
	>
		<option selected value={''}>{$_('.action.select')}</option>
		{#each items as item (item.id)}
			<option value={item.id}>{item.id} - {item.name} - {item.section}</option>
		{/each}
	</select>
</label>
