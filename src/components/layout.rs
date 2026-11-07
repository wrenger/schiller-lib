use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    router::{href, request::uri},
    runtime::{Event, Signal, link, signal},
    view::{Child, View, attributes, class, component, view},
};

use crate::{
    components::{
        button::{ButtonSize, ButtonVariant, button},
        separator::{SeparatorOrientation, separator},
        sidebar::{
            SidebarCollapsible, SidebarMenuButtonSize, SidebarMenuButtonVariant, sidebar,
            sidebar_content, sidebar_footer, sidebar_group, sidebar_group_content,
            sidebar_group_label, sidebar_header, sidebar_header_slim, sidebar_inset, sidebar_menu,
            sidebar_menu_button_variants, sidebar_menu_item, sidebar_provider, sidebar_trigger,
        },
    },
    util::FAVICON,
};

pub const SIDEBAR_ID: &'static str = "schiller-lib-sidebar";

struct NavItem {
    label: &'static str,
    icon: IconData,
    target: &'static str,
}

static NAV: [NavItem; 4] = [
    NavItem {
        label: "Books",
        icon: iconify_icon!("lucide:book-marked"),
        target: "/books",
    },
    NavItem {
        label: "Users",
        icon: iconify_icon!("lucide:user"),
        target: "/users",
    },
    NavItem {
        label: "Overdues",
        icon: iconify_icon!("lucide:calendar-clock"),
        target: "/overdues",
    },
    NavItem {
        label: "Info",
        icon: iconify_icon!("lucide:info"),
        target: "/info",
    },
];

fn in_section(cx: &Cx, path: &str) -> bool {
    let current = uri(cx).path();
    current == path
        || (path != "/"
            && current
                .strip_prefix(path)
                .is_some_and(|rest| rest.starts_with('/')))
}

/// TODO: Sheet for smaller list content displaying the display.
#[component]
pub async fn dividing_layout(
    cx: &Cx,
    list_header: Child<'_>,
    list: Child<'_>,
    display_header: Child<'_>,
    display: Child<'_>,
) -> Result<impl View> {
    let open = signal(cx, || true);
    let mobile_open = signal(cx, || false);

    Ok(view! {
        <body
            :data-sidebar-open=$(raw!(
                r#"(() => {
                    const stored = localStorage.getItem("schiller.sidebar.open");
                    const storedOpen = stored === "true";
                    if (stored !== null && ${open}.get().dehydrate() !== storedOpen) {
                        ${open}.set(cx.hydrate(storedOpen));
                    }
                    return ${open}.get();
                })()"#,
                open.get(),
            ))
        >
            sidebar_provider(
                app_sidebar(id: SIDEBAR_ID, open: &open, mobile_open: &mobile_open)
                sidebar_inset(sidebar_header_slim(
                    <div class="grid grid-cols-1 md:grid-cols-[max(37%,275px)_auto_1fr] w-full">
                        <div class="flex h-14 items-center gap-3 px-4">
                            sidebar_trigger(
                                open: $(open.get()),
                                attrs: attributes! {
                                    class="max-md:hidden cursor-pointer"
                                    aria-controls=(SIDEBAR_ID)
                                    @click=$(|_e: Event| {
                                        open.toggle();
                                        raw!(r#"localStorage.setItem("schiller.sidebar.open", String(${open}.get()))"#);
                                    })
                                }
                            )
                            sidebar_trigger(
                                open: $(mobile_open.get()),
                                attrs: attributes! {
                                    class="md:hidden cursor-pointer"
                                    aria-controls=(&SIDEBAR_ID)
                                    @click=$(|_e: Event| mobile_open.toggle())
                                }
                            )
                            separator(orientation: SeparatorOrientation::Vertical, attrs: attributes! {
                                class="h-5!"
                            })
                            (list_header)
                        </div>

                        separator(orientation: SeparatorOrientation::Vertical, attrs: attributes! {
                            class="hidden md:block"
                        })

                        <div class="flex h-14 items-center gap-3 px-4 hidden md:block">
                            (display_header)
                        </div>
                    </div>

                )
                <div class="grid grid-cols-1 md:grid-cols-[max(37%,275px)_auto_1fr] h-full w-full">
                    <div class="p-4 gap-3">
                        (list)
                    </div>

                    separator(orientation: SeparatorOrientation::Vertical, attrs: attributes! {
                        class="hidden md:block"
                    })

                    <div class="p-4 gap-3 hidden md:block">
                        (display)
                    </div>
                </div>
                )
            )
        </body>
    })
}

#[component]
pub async fn simplified_layout(cx: &Cx, header: Child<'_>, main: Child<'_>) -> Result<impl View> {
    let open = signal(cx, || true);
    let mobile_open = signal(cx, || false);

    Ok(view! {
        <body
            :data-sidebar-open=$(raw!(
                r#"(() => {
                    const stored = localStorage.getItem("schiller.sidebar.open");
                    const storedOpen = stored === "true";
                    if (stored !== null && ${open}.get().dehydrate() !== storedOpen) {
                        ${open}.set(cx.hydrate(storedOpen));
                    }
                    return ${open}.get();
                })()"#,
                open.get(),
            ))
        >
            sidebar_provider(
                app_sidebar(id: SIDEBAR_ID, open: &open, mobile_open: &mobile_open)
                sidebar_inset(sidebar_header_slim(
                        <div class="flex h-14 items-center gap-3 px-4">
                            sidebar_trigger(
                                open: $(open.get()),
                                attrs: attributes! {
                                    class="max-md:hidden cursor-pointer"
                                    aria-controls=(SIDEBAR_ID)
                                    @click=$(|_e: Event| {
                                        open.toggle();
                                        raw!(r#"localStorage.setItem("schiller.sidebar.open", String(${open}.get()))"#);
                                    })
                                }
                            )
                            sidebar_trigger(
                                open: $(mobile_open.get()),
                                attrs: attributes! {
                                    class="md:hidden cursor-pointer"
                                    aria-controls=(&SIDEBAR_ID)
                                    @click=$(|_e: Event| mobile_open.toggle())
                                }
                            )
                            separator(orientation: SeparatorOrientation::Vertical, attrs: attributes! {
                                class="h-5!"
                            })
                            (header)
                        </div>
                    )
                <div class="p-4 h-full w-full">
                    (main)
                </div>
                )
            )
        </body>
    })
}

#[component]
async fn app_sidebar(
    cx: &Cx,
    id: &str,
    open: &Signal<bool>,
    mobile_open: &Signal<bool>,
) -> Result<impl View> {
    Ok(view! {
        sidebar(
            open: $(open.get()),
            mobile_open: $(mobile_open.get()),
            collapsible: SidebarCollapsible::Icon,
            sheet_attrs: attributes! {
                id=(id)
                aria-label="Schiller library navigation"
                @keydown=$(|e: Event| {
                    if e.key == "Escape" { mobile_open.set(false); }
                })
                @click=$(|e: Event| {
                    if e.target.id == id { mobile_open.set(false); }
                })
            },
            sidebar_header(
                <div class="flex items-center gap-1 py-2">
                    <a
                        href=(href!("/"))
                        class=(class!(sidebar_menu_button_variants(
                            SidebarMenuButtonVariant::Default,
                            SidebarMenuButtonSize::Lg,
                        ), "h-10"))
                    >
                        <img src=(FAVICON) class="h-6 w-6" />
                        <span class="font-semibold">"Schiller Library"</span>
                    </a>
                    button(
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Icon,
                        attrs: attributes! {
                            type="button"
                            class="md:hidden"
                            aria-label="Close sidebar"
                            @click=$(|_e: Event| mobile_open.set(false))
                        },
                        icon(data: iconify_icon!("lucide:x"))
                    )
                </div>
            )
            sidebar_content(
                sidebar_group(
                    sidebar_group_label("Library")
                    sidebar_group_content(
                        sidebar_menu(
                            for item in &NAV {
                                let target = href!(item.target);
                                let current = in_section(cx, &target.resolve(cx));
                                sidebar_menu_item(
                                    link(
                                        href: target,
                                        attrs: attributes! {
                                            data-sidebar="menu-button"
                                            data-active=(if current { "true" } else { "false" })
                                            aria-current=(current.then_some("page"))
                                            title=(item.label)
                                            class=(sidebar_menu_button_variants(
                                                SidebarMenuButtonVariant::Default,
                                                SidebarMenuButtonSize::Md,
                                            ))
                                            @click=$(|_e: Event| mobile_open.set(false))
                                        },
                                        icon(data: item.icon.clone())
                                        <span>(item.label)</span>
                                    )
                                )
                            }
                        )
                    )
                )
            )
            sidebar_footer(
                sidebar_menu(
                    sidebar_menu_item(
                        let target = href!("/settings");
                        let current = in_section(cx, &target.resolve(cx));
                        link(
                            href: target,
                            attrs: attributes! {
                                data-sidebar="menu-button"
                                title="Settings"
                                data-active=(if current { "true" } else { "false" })
                                aria-current=(current.then_some("page"))
                                class=(sidebar_menu_button_variants(
                                    SidebarMenuButtonVariant::Default,
                                    SidebarMenuButtonSize::Md,
                                ))
                                @click=$(|_e: Event| mobile_open.set(false))
                            },
                            icon(data: iconify_icon!("lucide:settings"))
                            <span>"Settings"</span>
                        )
                    )
                )
            )
        )
    })
}
