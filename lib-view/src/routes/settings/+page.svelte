<script lang="ts">
	import { _ } from 'svelte-i18n';
	import Layout from '../Layout.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import Local from './Local.svelte';
	import Global from './Global.svelte';
	import Separator from "$lib/components/ui/separator/separator.svelte";

	let scope = 'local';
</script>

<svelte:head>
	<title>{$_('.pref.title')}</title>
	<meta name="description" content={$_('.pref.title')} />
</svelte:head>

<Layout display={false}>
	<svelte:fragment slot="list-nav">
		<div class="flex h-full items-center justify-between px-4">
			<h1 class="text-xl font-bold">{$_('.pref.title')}</h1>
			<Tabs.Root bind:value={scope} class="w-1/2">
				<Tabs.List class="grid w-full grid-cols-2">
					<Tabs.Trigger value="local">{$_('.pref.local')}</Tabs.Trigger>
					<Tabs.Trigger value="global">{$_('.pref.global')}</Tabs.Trigger>
				</Tabs.List>
			</Tabs.Root>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="list">
		<div class="overflow-y-scroll md:container md:max-w-2xl mt-4">
			{#if scope == 'local'}
				<Local />
			{:else if scope == 'global'}
				<Global />
			{/if}
		</div>
	</svelte:fragment>
</Layout>
