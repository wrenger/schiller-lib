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
		--list-height: calc(100vh - 76px);
		display: grid;
		grid-template-areas: "list view";
		grid-template-rows: var(--list-height);
		grid-template-columns: 50% auto;
		height: 100%;
	}
	.list {
		grid-area: list;
		margin-right: 5px;
		display: flex;
		flex-direction: column;
	}
	.view {
		grid-area: view;
		overflow-y: scroll;
		margin-left: 5px;
	}
	@media only screen and (max-width: 768px) {
		.main {
			--list-height: 300px;
			grid-template-areas: "list" "view";
			grid-template-rows: var(--list-height) auto;
			grid-template-columns: auto;
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
