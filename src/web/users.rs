use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

use crate::components::layout::dividing_layout;

#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        dividing_layout(
            list_header: view! { <span>"Users list header"</span> }.into(),
            list: view! { <p class="p">"Users list"</p> }.into(),
            display_header: view! { <span>"Users display header"</span> }.into(),
            display: view! { <h1 class="h1">"Users display"</h1> }.into()
        )
    })
}
