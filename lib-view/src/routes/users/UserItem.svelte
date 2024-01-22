<script lang="ts" context="module">
	export const HEIGHT: number = 64;
</script>

<script lang="ts">
	import type api from '$lib/api';

	export let user: api.User;
	export let active: boolean;
	export let onClick: () => void;

	$: classesActive = (cond: boolean) => (cond ? '!bg-primary-active-token' : '');
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="list-option {classesActive(active)}"
	id={user.account}
	on:click={onClick}
	style="border-radius: 12px;"
>
	<span class="flex-auto truncate w-[200px]">
		<dt>{user.forename} {user.surname}</dt>
		<dd class="text-sm opacity-50">{user.account}</dd>
	</span>
	<span class="text-end truncate">
		<dt>{user.role}</dt>
	</span>
</div>
