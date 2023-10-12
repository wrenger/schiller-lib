<script lang="ts">
	import { _ } from "svelte-i18n";
	import { DateTime } from "luxon";
	export let date: DateTime;
	export let id = "";
	let minDate = DateTime.now().toISODate();

	let inputElement: HTMLInputElement | null = null;

	const updateDate = () => {
		if (inputElement) {
			const inputValue = inputElement.value;
			date = DateTime.fromFormat(inputValue, "yyyy-MM-dd");
		}
	};
</script>

<input
	{id}
	min={minDate}
	type="date"
	class="form-control"
	placeholder={$_(".book.lend.period")}
	aria-label={$_(".book.lend.period")}
	value={date.toISODate()}
	bind:this={inputElement}
	on:input={updateDate}
/>
