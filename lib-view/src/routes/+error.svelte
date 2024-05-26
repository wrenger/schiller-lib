<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { page } from '$app/stores';
	import CircleAlert from 'lucide-svelte/icons/circle-alert';
	import * as Alert from '$lib/components/ui/alert';
</script>

<div class="h-fit p-4">
	<Alert.Root variant="destructive">
		<CircleAlert class="h-4 w-4" />
		<Alert.Title>{$_('.alert.error')}</Alert.Title>
		<Alert.Description>{$page.status} - {$page.error?.message}</Alert.Description>
	</Alert.Root>
</div>
