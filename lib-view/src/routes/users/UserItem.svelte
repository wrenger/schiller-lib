<script lang="ts">
	import type api from "$lib/api";

	export let user: api.User;
	export let active: boolean;
	export let onClick: () => void;
</script>

<button
	class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
	class:active
	id={user.account}
	on:click={onClick}
>
	<div class="d-flex flex-column">
		<p class="mb-0 text-truncate q">{user.forename} {user.surname}</p>
		<small class="text-muted text-truncate q">{user.account}</small>
	</div>
	<div class="d-flex flex-column align-items-end">
		<p class="mb-0 text-truncate q">{user.role}</p>
	</div>
</button>
