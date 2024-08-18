<script lang="ts">
	import { _ } from 'svelte-i18n';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import Button from '$lib/components/ui/button/button.svelte';
	import { categories } from '$lib/store';
	import api from '$lib/api';
	import { areObjectsEqual, handle_result } from '$lib';
	import Spinner from '$lib/components/ui/spinner/Spinner.svelte';

	let categoryMode = 'add';
	let emptyNew: api.Category = { id: '', name: '', section: '' };

	let selected: api.Category = $categories[0];

	let id: string;
	let name: string;
	let section: string;

	function selectCategory(item: api.Category) {
		selected = item;
		id = item.id;
		name = item.name;
		section = item.section;
	}

	$: selectCategory(selected);

	let addResponse: Promise<void>;
	async function add() {
		handle_result(await api.category_add(emptyNew));
		await onChange();
	}

	let editResponse: Promise<void>;
	async function edit() {
		handle_result(await api.category_update(selected.id, { id, name, section }));
		await onChange();
	}

	let deleteResponse: Promise<void>;
	async function del() {
		handle_result(await api.category_delete(selected.id));
		await onChange();
	}

	async function onChange() {
		selected = {
			id,
			name,
			section
		};
		await reload();
	}

	async function reload() {
		let data = handle_result(await api.category_list());
		categories.set(data);
		if (selected != null) {
			let sid = categoryMode == 'add' ? emptyNew.id : selected.id;
			selected = data.find((t) => t.id == sid) || $categories[0];
		}
		categoryMode = 'edit';
		emptyNew = { id: '', name: '', section: '' };
	}
</script>

<Tabs.Root bind:value={categoryMode} class="w-full" id="category">
	<Tabs.List class="grid w-full grid-cols-2">
		<Tabs.Trigger value="add">{$_('.action.add')}</Tabs.Trigger>
		<Tabs.Trigger value="edit">{$_('.action.edit')}</Tabs.Trigger>
	</Tabs.List>
	<Tabs.Content value="add">
		<div class="space-y-2">
			<div class="flex w-full flex-col gap-1.5">
				<Label for="id">{$_('.category.id')}</Label>
				<Input bind:value={emptyNew.id} type="text" id="id" placeholder={$_('.category.id')} />
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="name">{$_('.category.name')}</Label>
				<Input
					bind:value={emptyNew.name}
					type="text"
					id="name"
					placeholder={$_('.category.name')}
				/>
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="section">{$_('.category.section')}</Label>
				<Input
					bind:value={emptyNew.section}
					type="text"
					id="section"
					placeholder={$_('.category.section')}
				/>
			</div>
			<Button
				class="w-full"
				disabled={!(emptyNew.id && emptyNew.name && emptyNew.section)}
				on:click={() => (addResponse = add())}
			>
				<Spinner response={addResponse} />
				{$_('.action.add')}
			</Button>
		</div>
	</Tabs.Content>
	<Tabs.Content value="edit">
		<div class="space-y-2">
			<div class="flex w-full flex-col gap-1.5">
				<Label for="category-list">{$_('.category')}</Label>
				<Select.Root
					selected={{
						value: selected.id,
						label: `${selected.id} - ${selected.name} - ${selected.section}`
					}}
				>
					<Select.Trigger class="w-full" id="category-list">
						<Select.Value placeholder={$_('.category')} />
					</Select.Trigger>
					<Select.Content>
						<div class="max-h-72 overflow-y-scroll">
							{#each $categories as category}
								<Select.Item
									on:click={() => {
										selectCategory(category);
									}}
									value={category.id}
									>{category.id} - {category.name} - {category.section}</Select.Item
								>
							{/each}
						</div>
					</Select.Content>
				</Select.Root>
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="id">{$_('.category.id')}</Label>
				<Input bind:value={id} type="text" id="id" placeholder={$_('.category.id')} />
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="name">{$_('.category.name')}</Label>
				<Input bind:value={name} type="text" id="name" placeholder={$_('.category.name')} />
			</div>
			<div class="flex w-full flex-col gap-1.5">
				<Label for="section">{$_('.category.section')}</Label>
				<Input
					bind:value={section}
					type="text"
					id="section"
					placeholder={$_('.category.section')}
				/>
			</div>
			<div class="grid grid-cols-2 gap-2">
				<Button
					class="w-full"
					disabled={areObjectsEqual(selected, { id, name, section })}
					on:click={() => (editResponse = edit())}
				>
					<Spinner response={editResponse} />
					{$_('.action.apply')}
				</Button>
				<Button class="w-full" variant="destructive" on:click={() => (deleteResponse = del())}>
					<Spinner response={deleteResponse} />
					{$_('.action.delete')}
				</Button>
			</div>
		</div>
	</Tabs.Content>
</Tabs.Root>
