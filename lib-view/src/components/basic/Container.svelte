<script lang="ts">
	export let isActive: boolean;
</script>

<div class="main">
	<div class="list" class:hidden={isActive}>
		<slot name="list" />
	</div>
	<div class="view" class:hidden={!isActive}>
		<slot name="view" />
	</div>
</div>

<style>
	.main {
		display: grid;
		grid-template-areas: 'list view';
		grid-template-rows: 100%;
		grid-template-columns: 50% auto;
		height: 100%;
	}
	.list {
		grid-area: list;
		margin-right: 5px;
	}
	.view {
		grid-area: view;
		margin-left: 5px;
		overflow-y: scroll;
		overflow-x: hidden;
	}
	.list.hidden,
	.view.hidden {
		display: block;
	}
	@media only screen and (max-width: 768px) {
		.main {
			display: flex;
		}
		.list,
		.view {
			width: 100%;
			margin: 0;
		}
		.list.hidden,
		.view.hidden {
			display: none;
		}
	}
</style>
