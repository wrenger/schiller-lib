<script lang="ts">
	import { _ } from "svelte-i18n";
	import { page } from "$app/stores";
	import Settings from "../settings/Settings.svelte";
</script>

<nav class="navbar navbar-expand bg-body-tertiary">
	<div class="container-fluid">
		<div class="navbar-nav">
			<a class={"nav-link"} class:active={$page.url.pathname === "/books"} href="/books">
				<div class="m-0 fs-6 text-truncate text-mobile">
					<i class="bi bi-book" title={$_(".search.book")} />
					{$_(".search.book")}
				</div>
			</a>
			<a class={"nav-link"} class:active={$page.url.pathname === "/users"} href="/users">
				<div class="m-0 fs-6 text-truncate text-mobile">
					<i class="bi bi-people" title={$_(".search.user")} />
					{$_(".search.user")}
				</div>
			</a>
			<a class={"nav-link"} class:active={$page.url.pathname === "/overdues"} href="/overdues">
				<div class="m-0 fs-6 text-truncate text-mobile">
					<i class="bi bi-clock" title={$_(".book.overdues")} />
					{$_(".book.overdues")}
				</div>
			</a>
			<a class={"nav-link"} class:active={$page.url.pathname === "/info"} href="/info">
				<div class="m-0 fs-6 text-truncate text-mobile">
					<i class="bi bi-info-circle" title={$_(".alert.info")} />
					{$_(".alert.info")}
				</div>
			</a>
		</div>
		<Settings />
	</div>
</nav>
