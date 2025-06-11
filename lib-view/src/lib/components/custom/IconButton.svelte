<script lang="ts">
	import { FileQuestion, Icon } from 'lucide-svelte';
	import { Button, buttonVariants } from '../ui/button';
	import * as Tooltip from '../ui/tooltip';
	import type { VariantProps } from 'tailwind-variants';
	import type { TooltipContentProps, Builder } from 'bits-ui';
	import type { ComponentType } from 'svelte';
	import { cn } from '$lib/utils.js';

	export let icon: ComponentType<Icon> = FileQuestion;
	export let label: string = '';
	export let extra: string = '';

	export let onClick: (() => void) | undefined = undefined;
	export let builders: Builder[] = [];
	export let href: string | undefined = undefined;

	export let variant: VariantProps<typeof buttonVariants>['variant'] = 'ghost';
	let className: string = '';
	export { className as class };

	export let disabled: boolean = false;
	export let tooltip_side: TooltipContentProps['side'] = undefined;
</script>

<Tooltip.Root openDelay={0}>
	<Tooltip.Trigger asChild let:builder={tooltip}>
		<Button
			on:click={onClick}
			{href}
			{variant}
			size="icon"
			class={cn('rounded-lg', className)}
			aria-label={label}
			builders={[...builders, tooltip]}
			{disabled}
		>
			<svelte:component this={icon} class="size-5" />
		</Button>
	</Tooltip.Trigger>
	<Tooltip.Content side={tooltip_side} sideOffset={5}>
		{label}
		{#if extra}
			<span class="text-muted-foreground">{extra}</span>
		{/if}
	</Tooltip.Content>
</Tooltip.Root>
