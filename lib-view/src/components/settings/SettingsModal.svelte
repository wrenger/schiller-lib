<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { Tab, TabGroup, getModalStore } from '@skeletonlabs/skeleton';
	import Local from './Local.svelte';
	import Global from './Global.svelte';
	import api from '../../lib/api';
	import { areObjectsEqual } from '$lib/util';
	import Spinner from '../basic/Spinner.svelte';

	// Stores
	import { settingsGlobal, state } from '$lib/store';
	import type { SvelteComponent } from 'svelte';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	let tab = 0;
	let gl: Global;

	const modalStore = getModalStore();

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4 relative';
	const cTabs = 'max-w-5xl mx-auto hide-scrollbar'; // sticky top-0 z-[1]

	// settings requesting
	let saveResponse: Promise<void>;
	async function save() {
		let settings = gl.get();
		let data = {
			...settings,
			mail_last_reminder: settings?.mail_last_reminder.toISODate() ?? ''
		};

		if (!areObjectsEqual(settings, $settingsGlobal)) {
			await api.settings_update(data);
			if (settings) settingsGlobal.set(settings);
			state.set({});
		}
	}
</script>

<!-- @component This is the settings modal. -->

{#if $modalStore[0]}
	<div class={cBase}>
		<TabGroup class={cTabs} style="outline: none;">
			<Tab bind:group={tab} name="tab1" value={0}>{$_('.pref.local')}</Tab>
			<Tab bind:group={tab} name="tab2" value={1}>{$_('.pref.global')}</Tab>
		</TabGroup>
		<!-- Tab Panels --->
		<div id="panels" class="space-y-4">
			{#if tab === 0}
				<Local />
			{:else if tab === 1}
				<Global bind:this={gl} />
			{/if}
		</div>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{$_('.action.close')}</button>
		{#if tab === 1}
       		<button class="btn {parent.buttonPositive}" on:click={async () => saveResponse = save()}><Spinner response={saveResponse} />{$_(".action.apply")}</button>
		{/if}
    </footer>
	</div>
{/if}
