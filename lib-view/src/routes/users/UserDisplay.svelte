<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type api from '$lib/api';
	import { Badge } from '$lib/components/ui/badge';

	export let user: api.User;

	function displayInformation(info: string | number | undefined) {
		return info ? info : $_('.action.empty');
	}
</script>

<div class="h-full w-full space-y-4 overflow-y-scroll p-4">
	<div class="border-b pb-3">
		<h2 class="text-xl font-semibold">{user.forename} {user.surname}</h2>
		<p class="text-md">
			{user.account}
		</p>
	</div>
	<div>
		<h3 class="text-sm text-muted-foreground">{$_('.user.role')}</h3>
		<p class="text-md font-medium">{displayInformation(user.role)}</p>
	</div>
	<div class="flex items-center space-x-2">
		<span class="text-md font-medium">{$_('.user.may-borrow')}:</span>
		{#if user.may_borrow}
			<Badge>{$_('.action.yes')}</Badge>
		{:else}
			<Badge>{$_('.action.no')}</Badge>
		{/if}
	</div>
</div>
