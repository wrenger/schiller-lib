<script lang="ts">
	export var isActive: boolean;
</script>

<section class="main">
	<div class="list" class:shown={isActive}>
		<slot name="list" />
	</div>
	<div class="view" class:shown={!isActive}>
		<slot name="view" />
	</div>
</section>

<style>
	.main {
		display: grid;
		grid-template-areas: "list view";
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
		overflow-y: scroll;
		overflow-x: hidden;
		margin-left: 5px;
	}
	@media only screen and (max-width: 768px) {
		.main {
			display: flex;
		}
		.list {
			width: 100%;
			margin: 0;
		}
		.view {
			width: 100%;
			margin: 0;
		}
		.list.shown {
			display: none;
		}
		.view.shown {
			display: none;
		}
	}
</style>
