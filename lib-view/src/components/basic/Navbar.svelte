<script lang="ts">
    import { _ } from "svelte-i18n";
    import { page } from "$app/stores";
    import Settings from "../settings/Settings.svelte";
</script>

<nav class="navbar navbar-expand-sm bg-body-tertiary">
    <div class="container-fluid">
        <button
            class="navbar-toggler"
            type="button"
            data-bs-toggle="offcanvas"
            data-bs-target="#offcanvasNavbar"
            aria-controls="offcanvasNavbar"
            aria-label="Toggle navigation"
        >
            <span class="navbar-toggler-icon" />
        </button>
        <div
            class="offcanvas offcanvas-end"
            tabindex="-1"
            id="offcanvasNavbar"
            aria-labelledby="offcanvasNavbarLabel"
        >
            <div class="offcanvas-header">
                <h5 class="offcanvas-title" id="offcanvasNavbarLabel">Menu</h5>
                <button type="button" class="btn-close" data-bs-dismiss="offcanvas" aria-label="Close" />
            </div>
            <div class="offcanvas-body">
                <div class="navbar-nav flex-grow-1 pe-3">
                    <a class={"nav-link"} class:active={$page.url.pathname === "/books"} href="/books">
                        {$_(".search.book")}
                    </a>
                    <a class={"nav-link"} class:active={$page.url.pathname === "/users"} href="/users">
                        {$_(".search.user")}
                    </a>
                    <a class={"nav-link"} class:active={$page.url.pathname === "/overdues"} href="/overdues">
                        {$_(".book.overdues")}
                    </a>
                    <a class={"nav-link"} class:active={$page.url.pathname === "/info"} href="/info">
                        {$_(".alert.info")}
                    </a>
                </div>
            </div>
        </div>
        <Settings />
    </div>
</nav>
