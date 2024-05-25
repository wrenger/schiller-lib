<script lang="ts" context="module">
	export const HEIGHT: number = 70;
</script>

<script lang="ts">
	import type api from '$lib/api';
	import { cn } from '$lib/utils';
	import { _ } from 'svelte-i18n';

	export let user: api.User;
	export let active: boolean = false;
	export let onClick: () => void;

	$: buttonActive = active ? 'bg-muted' : '';
</script>

<button
	class={cn(
		`hover:bg-accent flex h-[62px] w-full flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all ${buttonActive}`
	)}
	id={user.account}
	on:click={onClick}
>
	<div class="grid w-full grid-cols-[1fr_auto] gap-1">
		<div class="w-full overflow-hidden">
			<div class="truncate font-semibold">{user.forename} {user.surname}</div>
			<div class={cn(`text-muted-foreground ml-auto text-nowrap text-xs`)}>{user.account}</div>
		</div>
		<div class="flex items-center truncate text-xs font-medium">{user.role}</div>
	</div>
</button>
