use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

use crate::components::layout::simplified_layout;

#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        simplified_layout(
            header: view! { <span>"Overdues header"</span> }.into(),
            main: view! { <p class="p">"Overdues main"</p> }.into()
        )
    })
}
