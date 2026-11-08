use crate::t;

use topcoat::{
    Result,
    context::{Cx, app_context},
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
    util::about,
};

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let login = crate::server::require_login(cx).await?;
    let db: &AtomicDatabase = app_context(cx);
    let login_id = login.id.clone();
    let about = about();
    let stats = db.read().stats()?;
    let stats_arr = [
        t!(info_books, arg0 = stats.books),
        t!(info_users, arg0 = stats.users),
        t!(info_categories, arg0 = stats.categories),
        t!(info_borrows, arg0 = stats.borrows),
        t!(info_reservations, arg0 = stats.reservations),
        t!(info_overdues, arg0 = stats.overdues),
    ];

    Ok(view! {
        simplified_layout(
            header: view! { <h4 class="h4">(t!(main_info))</h4> }.into(),
            main: view! {
                <div class="space-y-2">
                    <h5 class="h5">(t!(info_session))</h5>
                    separator(orientation: SeparatorOrientation::Horizontal)
                    <p class="p">
                        (t!(info_session_id, arg0 = login_id))
                    </p>
                </div>
                <div class="grid gap-4 pt-4 md:grid-cols-2">
                    <div class="space-y-2">
                        <h5 class="h5">(t!(info_stats))</h5>
                        separator(orientation: SeparatorOrientation::Horizontal)
                        <div class="space-y-2 p-2 pb-0">
                            for label in stats_arr {
                                <div class="flex items-center">
                                    icon(
                                        data: iconify_icon!("lucide:circle-chevron-right"),
                                        attrs: attributes! { class="mr-3" }
                                    )
                                    <span class="flex-auto">
                                        (label)
                                    </span>
                                </div>
                            }
                        </div>
                    </div>
                    <div class="space-y-2">
                        <h5 class="h5">
                            (t!(info_about, arg0 = about.name, arg1 = about.version))
                        </h5>
                        separator(orientation: SeparatorOrientation::Horizontal)
                        <p class="p">
                            (t!(page_description, arg0 = about.description))
                        </p>
                        <p class="p">
                            (t!(info_about_repo))
                            " "
                            <a
                                class="text-primary underline underline-offset-4"
                                href=(about.repository)
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                (about.repository)
                            </a>
                        </p>
                        <p class="p">(t!(book_authors))</p>
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
                            (t!(info_about_license, arg0 = about.license))
                        </p>
                    </div>
                </div>
            }.into(

            )
        )
    })
}
