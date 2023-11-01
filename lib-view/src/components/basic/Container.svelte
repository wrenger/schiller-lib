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
			grid-template-areas: "list" "view";
			grid-template-rows: 50% auto;
			grid-template-columns: auto;
		}
		.list {
			margin-right: 0px;
			padding-bottom: 5px;
		}
		.view {
			padding-top: 5px;
			margin-left: 0px;
		}
	}
</style>
