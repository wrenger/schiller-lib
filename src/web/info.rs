use topcoat::{
    Result,
    context::{Cx, app_context, request_context},
    icon::{icon, iconify::iconify_icon},
    router::page,
    view::{View, attributes, view},
};

use crate::{
    components::{
        layout::simplified_layout,
        separator::{SeparatorOrientation, separator},
    },
    db::AtomicDatabase,
    server::Login,
    util::about,
};

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let db: &AtomicDatabase = app_context(cx);
    let login: &Login = request_context(cx);
    let login_id = login.id.clone();
    let about = about();
    let stats = db.read().stats()?;
    let stats_arr = [
        (stats.books, "books"),
        (stats.users, "users"),
        (stats.categories, "categories"),
        (stats.borrows, "borrows"),
        (stats.reservations, "reservations"),
        (stats.overdues, "overdues"),
    ];

    Ok(view! {
        simplified_layout(
            header: view! { <h4 class="h4">"Info"</h4> }.into(),
            main: view! {
                <div class="space-y-2">
                    <h5 class="h5">"Session"</h5>
                    separator(orientation: SeparatorOrientation::Horizontal)
                    <p class="p">
                        "ID: "
                        (login_id)
                    </p>
                </div>
                <div class="grid gap-4 pt-4 md:grid-cols-2">
                    <div class="space-y-2">
                        <h5 class="h5">"Statistics "</h5>
                        separator(orientation: SeparatorOrientation::Horizontal)
                        <div class="space-y-2 p-2 pb-0">
                            for (count, label) in stats_arr {
                                <div class="flex items-center">
                                    icon(
                                        data: iconify_icon!("lucide:circle-chevron-right"),
                                        attrs: attributes! { class="mr-3" }
                                    )
                                    <span class="flex-auto">
                                        (count)
                                        " "
                                        (label)
                                    </span>
                                </div>
                            }
                        </div>
                    </div>
                    <div class="space-y-2">
                        <h5 class="h5">
                            "About "
                            (about.name)
                            " - Version "
                            (about.version)
                        </h5>
                        separator(orientation: SeparatorOrientation::Horizontal)
                        <p class="p">
                            "Description: "
                            (about.description)
                        </p>
                        <p class="p">
                            "Repository: "
                            <a
                                class="text-primary underline underline-offset-4"
                                href=(about.repository)
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                (about.repository)
                            </a>
                        </p>
                        <p class="p">"Authors: "</p>
                        <div class="space-y-2 p-2">
                            for author in about.authors {
                                <div class="flex items-center">
                                    icon(
                                        data: iconify_icon!("lucide:circle-chevron-right"),
                                        attrs: attributes! { class="mr-3" }
                                    )
                                    <span class="flex-auto">(author)</span>
                                </div>
                            }
                        </div>
                        <p class="p">
                            "License: "
                            (about.license)
                        </p>
                    </div>
                </div>
            }.into(

            )
        )
    })
}
