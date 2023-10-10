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
		overflow: scroll;
		margin-left: 5px;
	}
	@media only screen and (max-width: 768px) {
		.main {
			grid-template-areas: "list" "view";
			grid-template-rows: 300px auto;
			grid-template-columns: auto;
		}
		.list {
			margin-right: 0px;
		}
		.view {
			overflow: visible;
			padding-top: 15px;
			padding-bottom: 10px;
			margin-left: 0px;
		}
	}
</style>
