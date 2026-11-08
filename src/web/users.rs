use crate::t;

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::components::layout::dividing_layout;

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    crate::server::require_login(cx).await?;
    Ok(view! {
        dividing_layout(
            list_header: view! { <span>(t!(search_user))</span> }.into(),
            list: view! { <p class="p">(t!(page_placeholder))</p> }.into(),
            display_header: view! { <span>(t!(user))</span> }.into(),
            display: view! { <h1 class="h1">(t!(page_placeholder))</h1> }.into()
        )
    })
}
