use crate::t;

use topcoat::{
    Result,
    context::{Cx, app_context},
    icon::{icon, iconify::iconify_icon},
    router::page,
    view::{View, attributes, view},
};

use crate::components::{
    book_view::book_view,
    button::{ButtonSize, ButtonVariant, button},
    input::input,
    layout::dividing_layout,
    separator::{SeparatorOrientation, separator},
};
use crate::db::AtomicDatabase;

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    crate::server::require_login(cx).await?;
    let (book, category) = {
        let db = app_context::<AtomicDatabase>(cx).read();
        let book = db.books.fetch("demo AACH 1").map_err(|error| {
            topcoat::Error::from(error).context(
                "Could not load demo book 'demo AACH 1'; use --db test/demo.json for the books preview",
            )
        })?;
        let category = db.categories.data.get(&book.category).cloned();
        (book, category)
    };

    Ok(view! {
        dividing_layout(
            list_header: view! {
                <div class="flex justify-between w-full items-center">
                    <h4 class="h4">(t!(search_book))</h4>
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" aria-label=(t!(book_not_borrowable)) },
                            icon(data: iconify_icon!("lucide:book-dashed"))
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" aria-label=(t!(category_edit)) },
                            icon(data: iconify_icon!("lucide:tags"))
                        )
                        separator(
                            orientation: SeparatorOrientation::Vertical,
                            attrs: attributes! { class="h-5!" }
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" aria-label=(t!(book_new)) },
                            icon(data: iconify_icon!("lucide:plus"))
                        )
                    </div>
                </div>
            }.into(

            ),
            list: view! {
                <div class="flex flex-col gap-4 w-full">
                    input(attrs: attributes! { type="search" placeholder=(t!(search_action)) aria-label=(t!(search_action)) })
                    <div class="flex flex-col gap-2">(t!(page_placeholder))</div>
                </div>
            }.into(

            ),
            display_header: view! {
                <div class="flex justify-between w-full h-full items-center">
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" aria-label=(t!(book_lend)) },
                            icon(data: iconify_icon!("lucide:upload"))
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" disabled="true" aria-label=(t!(book_revoke)) },
                            icon(data: iconify_icon!("lucide:import"))
                        )
                        separator(
                            orientation: SeparatorOrientation::Vertical,
                            attrs: attributes! { class="h-5!" }
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" disabled="true" aria-label=(t!(book_delete_reservation)) },
                            icon(data: iconify_icon!("lucide:bookmark-x"))
                        )
                    </div>
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" aria-label=(t!(action_edit)) },
                            icon(data: iconify_icon!("lucide:pencil"))
                        )
                        separator(
                            orientation: SeparatorOrientation::Vertical,
                            attrs: attributes! { class="h-5!" }
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" class="text-red-500" aria-label=(t!(action_delete)) },
                            icon(data: iconify_icon!("lucide:trash-2"))
                        )
                    </div>
                </div>
            }.into(

            ),
            display: view! {
                // https://www.sra.uni-hannover.de/news.html#news-2026-norcas-thomas
                // TODO List highlighting mit :target
                // Put #signatur in the url, show corresponding book
                book_view(book: book, category: category)
            }.into(

            )
        )
    })
}
