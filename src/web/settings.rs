use crate::t;

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::components::layout::simplified_layout;

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    crate::server::require_login(cx).await?;
    Ok(view! {
        simplified_layout(
            header: view! { <span>(t!(pref_title))</span> }.into(),
            main: view! { <p class="p">(t!(page_placeholder))</p> }.into()
        )
    })
}
