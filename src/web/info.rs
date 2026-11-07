use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::{components::layout::simplified_layout, db::AtomicDatabase};

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let db: &AtomicDatabase = topcoat::context::app_context(cx);
    let stats = db.read().stats()?;
    Ok(view! {
        simplified_layout(
            header: view! { <span>"Info header"</span> }.into(),
            main: view! { <p class="p">"Info main " (stats.books)</p> }.into()
        )
    })
}
