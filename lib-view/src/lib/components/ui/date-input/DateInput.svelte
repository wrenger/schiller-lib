<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { DateTime } from 'luxon';
	import { Input } from '../input';
	import { Label } from '../label';
	export let date: DateTime;
	export let min = true;
	export let label = '';
	export let labelClass = '';
	let minDate = DateTime.now().toISODate();

	const updateDate = (event: Event) => {
		const input = event.target as HTMLInputElement;
		date = DateTime.fromISO(input.value);
	};
</script>

<div class="flex w-full flex-col gap-1.5">
	<Label class={labelClass + ' text-left'} for={label.toLowerCase()}>{label}</Label>
	<Input
		min={min ? minDate : ''}
		placeholder={label}
		aria-label={label}
		id={label.toLowerCase()}
		value={date.isValid ? date.toISODate() : ''}
		on:input={updateDate}
		type="date"
	/>
</div>
