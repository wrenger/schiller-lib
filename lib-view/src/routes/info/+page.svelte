<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { onMount } from 'svelte';
	import api from '$lib/api';

	const header = 'p-2 pb-0 font-bold';

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

<div class="space-y-2">
	<div class="w-full max-h-full text-token card p-2 space-y-2 overflow-y-scroll">
		{#await session}
			<div class={header}>{$_('.action.load')}...</div>
			<hr />
			<div class="p-2 space-y-2">
				<div class="placeholder animate-pulse" />
				<div class="placeholder animate-pulse" />
			</div>
		{:then data}
			{#if data}
				<div class={header}>{$_('.info.session')}</div>
				<hr />
				<div class="p-2 space-y-2">
					<p>{$_('.info.session.id')} {data.id}</p>
					<p class="m-0">{$_('.info.session.username')} {data.username}</p>
				</div>
			{/if}
		{/await}
	</div>
	<div class="w-full grid grid-cols-1 md:grid-cols-2 gap-2">
		<div class="w-full max-h-full text-token card p-2 space-y-2 overflow-y-scroll">
			{#await stats}
				<div class={header}>{$_('.action.load')}...</div>
				<hr />
				<div class="p-2 space-y-2">
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
				</div>
			{:then data}
				{#if data}
					<div class={header}>{$_('.info.stats')}</div>
					<hr />
					<ul class="list p-2">
						{#each api.keys(data) as key, i (key)}
							<!-- ... -->
							<li>
								<span class="badge-icon p-2 variant-soft-primary"
									><i class="fa-solid fa-chevron-right"></i></span
								>
								<span class="flex-auto">{$_(`.info.${key}`, { values: { '0': data[key] } })}</span>
							</li>
						{/each}
					</ul>
				{/if}
			{/await}
		</div>

		<div class="w-full max-h-full text-token card p-2 space-y-2 overflow-y-scroll mb-2 md:mb-0">
			{#await about}
				<div class={header}>{$_('.action.load')}...</div>
				<hr />
				<div class="p-2 space-y-2">
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
					<div class="placeholder animate-pulse" />
				</div>
			{:then data}
				{#if data}
					<div class={header}>
						{$_('.info.about', { values: { '0': data.name, '1': data.version } })}
					</div>
					<hr />
					<div class="p-2 space-y-2">
						<p>{$_('.info.about.name')} {data.description}</p>
						<p>
							{$_('.info.about.repo')}
							<a href={data.repository} class="anchor" target="_blank">{data.repository}</a>
						</p>
						<p class="m-0">{$_('.info.about.devs')}</p>
						<ul class="list">
							{#each data.authors as author (author)}
								<li>
									<span class="badge-icon p-2 variant-soft-primary"
										><i class="fa-solid fa-chevron-right"></i></span
									>
									<span class="flex-auto">{author}</span>
								</li>
							{/each}
						</ul>
						<p class="m-0">{$_('.info.about.license', { values: { '0': data.license } })}</p>
					</div>
				{/if}
			{/await}
		</div>
	</div>
</div>
