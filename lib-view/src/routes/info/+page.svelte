<script lang="ts">
	import api from '$lib/api';
	import Layout from '../Layout.svelte';
	import { _ } from 'svelte-i18n';
	import { onMount } from 'svelte';
	import { ChevronRight } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import Separator from '$lib/components/ui/separator/separator.svelte';

	let stats: Promise<api.Stats>;
	let session: Promise<api.Session>;
	let about: Promise<api.About>;

	onMount(() => {
		session = api.session();
		stats = api.stats();
		about = api.about();
	});
</script>

<svelte:head>
	<title>{$_('.alert.info')}</title>
	<meta name="description" content={$_('.alert.info')} />
</svelte:head>

<Layout display={false}>
	<svelte:fragment slot="list-nav">
		<div class="flex h-full items-center px-4">
			<h1 class="text-xl font-bold">{$_('.alert.info')}</h1>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="list">
		<div class="h-full overflow-y-scroll">
			<div class="p-4">
				{#await session then session}
					{#if session}
						<div class="space-y-2">
							<h1 class="text-lg font-semibold tracking-tight">
								{$_('.info.session')}
							</h1>
							<Separator />
							<p>{$_('.info.session.id', { values: { '0': session.id } })}</p>
							<p>{$_('.info.session.username', { values: { '0': session.username } })}</p>
						</div>
					{/if}
				{/await}
				<div class="grid gap-4 pt-4 md:grid-cols-2">
					{#await stats then stats}
						{#if stats}
							<div class="space-y-2">
								<h1 class="text-lg font-semibold tracking-tight">
									{$_('.info.stats')}
								</h1>
								<Separator />
								<div>
									<ul class="space-y-2 p-2 pb-0">
										{#each Object.entries(stats) as [key, val] (key)}
											<li>
												<Badge class="mr-3 p-0.5"><ChevronRight class="size-3" /></Badge>
												<span class="flex-auto">{$_(`.info.${key}`, { values: { '0': val } })}</span
												>
											</li>
										{/each}
									</ul>
								</div>
							</div>
						{/if}
					{/await}
					{#await about then about}
						{#if about}
							<div class="space-y-2">
								<h1 class="text-lg font-semibold tracking-tight">
									{$_('.info.about', {
										values: { '0': about.name, '1': about.version }
									})}
								</h1>
								<Separator />
								<p>
									{$_('.info.about.name', { values: { '0': about.description } })}
								</p>

								<p>
									{$_('.info.about.repo')}
									<a
										href={about.repository}
										class="text-primary underline underline-offset-4"
										target="_blank">{about.repository}</a
									>
								</p>
								<p class="m-0">{$_('.info.about.devs')}</p>
								<ul class="space-y-2 p-2">
									{#each about.authors as author (author)}
										<li>
											<Badge class="mr-3 p-0.5"><ChevronRight class="size-3" /></Badge>
											<span class="flex-auto">{author}</span>
										</li>
									{/each}
								</ul>
								<p class="m-0">{$_('.info.about.license', { values: { '0': about.license } })}</p>
							</div>
						{/if}
					{/await}
				</div>
			</div>
		</div>
	</svelte:fragment>
</Layout>
