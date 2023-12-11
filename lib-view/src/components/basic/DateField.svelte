<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { DateTime } from 'luxon';
	export let date: DateTime;
	export let min = true;
	export let label = '';
	let minDate = DateTime.now().toISODate();

	let inputElement: HTMLInputElement | null = null;

	const updateDate = () => {
		if (inputElement) date = DateTime.fromISO(inputElement.value);
	};
</script>

<label class="label">
	<span>{label}</span>
	<input
		min={min ? minDate : ''}
		type="date"
		class="input"
		placeholder={$_('.book.lend.period')}
		aria-label={$_('.book.lend.period')}
		value={date.isValid ? date.toISODate() : ''}
		bind:this={inputElement}
		on:input={updateDate}
	/>
</label>
