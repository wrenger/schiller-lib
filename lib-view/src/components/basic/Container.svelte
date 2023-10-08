<section class="main">
	<div class="list">
		<slot name="list" />
	</div>
	<div class="view">
		<slot name="view" />
	</div>
</section>

<style>
	.main {
		--box: calc(100vh - 76px);
		display: grid;
		grid-template: "list view" var(--box) / 50% auto;
		height: 100%;
	}
	.list {
		grid-area: list;
		margin-right: 5px;
		display: flex;
		flex-direction: column;
	}
	.view {
		overflow-y: scroll;
		grid-area: view;
		margin-left: 5px;
	}
	@media only screen and (max-width: 768px) {
		.main {
			--box: 300px;
			grid-template:
				"list" var(--box)
				"view" auto / auto;
		}
		.list {
			margin-right: 0px;
		}
		.view {
			padding-top: 15px;
			margin-left: 0px;
		}
	}
</style>
