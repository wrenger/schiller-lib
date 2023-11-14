<script lang="ts">
	import { _, date } from "svelte-i18n";
	import Request from "../../components/basic/Request.svelte";
	import { onMount } from "svelte";
	import api from "$lib/api";

	let r: Request;

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
	<title>{$_(".alert.info")}</title>
	<meta name="description" content={$_(".alert.info")} />
</svelte:head>

<section class="h-100" style="overflow: scroll;">
	<Request bind:this={r} />
	<div class="card mb-2">
		{#await session}
			<div class="card-header">{$_(".action.load")}...</div>
			<div class="card-body d-flex justify-content-center">
				<div class="spinner-grow" role="status">
					<span class="visually-hidden">Loading...</span>
				</div>
			</div>
		{:then data}
			{#if data}
				<div class="card-header">{$_(".info.session")}</div>
				<div class="card-body">
					<p>{$_(".info.session.id")} {data.id}</p>
					<p class="m-0">{$_(".info.session.username")} {data.username}</p>
				</div>
			{/if}
		{/await}
	</div>
	<div class="row m-0">
		<div class="col-md-6 mb-2 mb-md-0 p-0 pe-md-2">
			<div class="card h-100">
				{#await stats}
					<div class="card-header">{$_(".action.load")}...</div>
					<div class="card-body d-flex justify-content-center">
						<div class="spinner-grow" role="status">
							<span class="visually-hidden">Loading...</span>
						</div>
					</div>
				{:then data}
					{#if data}
						<div class="card-header">{$_(".info.stats")}</div>
						<ul class="list-group list-group-flush">
							{#each api.keys(data) as key (key)}
								<li class="list-group-item">
									{$_(`.info.${key}`, { values: { "0": data[key] } })}
								</li>
							{/each}
						</ul>
					{/if}
				{/await}
			</div>
		</div>
		<div class="col-md-6 p-0">
			<div class="card h-100">
				{#await about}
					<div class="card-header">{$_(".action.load")}...</div>
					<div class="card-body d-flex justify-content-center">
						<div class="spinner-grow" role="status">
							<span class="visually-hidden">Loading...</span>
						</div>
					</div>
				{:then data}
					{#if data}
						<div class="card-header">
							{$_(".info.about", { values: { "0": data.name, "1": data.version } })}
						</div>
						<div class="card-body">
							<p>{$_(".info.about.name")} {data.description}</p>
							<p>
								{$_(".info.about.repo")}
								<a href={data.repository} target="_blank">{data.repository}</a>
							</p>
							<p class="m-0">{$_(".info.about.devs")}</p>
							<ul>
								{#each data.authors as author (author)}
									<li>{author}</li>
								{/each}
							</ul>
							<p class="m-0">{$_(".info.about.license", { values: { "0": data.license } })}</p>
						</div>
					{/if}
				{/await}
			</div>
		</div>
	</div>
</section>
