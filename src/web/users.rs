use crate::t;

use topcoat::{
    Result,
    context::{Cx, app_context},
    icon::{icon, iconify::iconify_icon},
    router::page,
    view::{View, attributes, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    layout::dividing_layout,
    separator::{SeparatorOrientation, separator},
    user_view::user_view,
};
use crate::db::AtomicDatabase;

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    crate::server::require_login(cx).await?;
    let user = {
        let db = app_context::<AtomicDatabase>(cx).read();
        db.users.fetch("aaminah.seymour").map_err(|error| {
            topcoat::Error::from(error).context(
                "Could not load demo user 'aaminah.seymour'; use --db test/demo.json for the users preview",
            )
        })?
    };

    Ok(view! {
        dividing_layout(
            list_header: view! { <span>(t!(search_user))</span> }.into(),
            list: view! { <p class="p">(t!(page_placeholder))</p> }.into(),
            display_header: view! {
                <div class="flex justify-between w-full h-full items-center">
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" aria-label=(t!(user_books)) },
                            icon(data: iconify_icon!("lucide:book-copy"))
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
            }.into(),
            display: view! { user_view(user: user) }.into()
        )
    })
}
