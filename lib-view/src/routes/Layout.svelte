<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable';
	import * as Sheet from '$lib/components/ui/sheet';
	import Separator from '$lib/components/ui/separator/separator.svelte';

	export let display: boolean = true;
	export let open: boolean = false;
	let innerWidth: number = 0;

	$: if (!(innerWidth < 768)) open = false;

	export function setOpen(value: boolean) {
		if (innerWidth < 768) {
			open = value;
		}
	}
</script>

<svelte:window bind:innerWidth />

<Resizable.PaneGroup direction="horizontal" autoSaveId="layout">
	<Resizable.Pane defaultSize={40} minSize={30}>
		<div class="grid h-full w-full grid-rows-[59px_1px_auto]">
			<slot name="list-nav" />
			<Separator />
			<slot name="list" />
		</div>
	</Resizable.Pane>
	{#if display}
		<Resizable.Handle withHandle class="hidden md:flex" />
		<Resizable.Pane defaultSize={60} minSize={40} class="hidden md:block">
			<div class="grid h-full w-full grid-rows-[59px_1px_auto]">
				<slot name="display-nav" />
				<Separator />
				<slot name="display" />
			</div>
		</Resizable.Pane>
		<Sheet.Root bind:open onOpenChange={(value) => (open = value)}>
			<Sheet.Content class="w-full overflow-x-hidden overflow-y-scroll md:hidden">
				<Sheet.Header>
					<slot name="display-nav" />
					<Separator />
					<div class="text-left">
						<slot name="display" />
					</div>
				</Sheet.Header>
			</Sheet.Content>
		</Sheet.Root>
	{/if}
</Resizable.PaneGroup>
