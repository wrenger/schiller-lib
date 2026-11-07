use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    router::page,
    view::{View, attributes, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    input::input,
    layout::dividing_layout,
    separator::{SeparatorOrientation, separator},
};

#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        dividing_layout(
            list_header: view! {
                <div class="flex justify-between w-full items-center">
                    <h4 class="h4">"Books"</h4>
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" },
                            icon(data: iconify_icon!("lucide:book-dashed"))
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" },
                            icon(data: iconify_icon!("lucide:tags"))
                        )
                        separator(
                            orientation: SeparatorOrientation::Vertical,
                            attrs: attributes! { class="h-5!" }
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" },
                            icon(data: iconify_icon!("lucide:plus"))
                        )
                    </div>
                </div>
            }.into(),
            list: view! {
                <div class="flex flex-col gap-4 w-full">
                    input(attrs: attributes! { type="search" placeholder="Search..." })
                    <div class="flex flex-col gap-2">
                        "TODO"
                    </div>
                </div>
            }.into(),
            display_header: view! {
                <div class="flex justify-between w-full h-full items-center">
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" },
                            icon(data: iconify_icon!("lucide:upload"))
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" disabled="true" },
                            icon(data: iconify_icon!("lucide:import"))
                        )
                        separator(
                            orientation: SeparatorOrientation::Vertical,
                            attrs: attributes! { class="h-5!" }
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" disabled="true" },
                            icon(data: iconify_icon!("lucide:bookmark-x"))
                        )
                    </div>
                    <div class="flex justify-center items-center gap-3">
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" },
                            icon(data: iconify_icon!("lucide:pencil"))
                        )
                        separator(
                            orientation: SeparatorOrientation::Vertical,
                            attrs: attributes! { class="h-5!" }
                        )
                        button(
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            attrs: attributes! { type="button" class="text-red-500" },
                            icon(data: iconify_icon!("lucide:trash-2"))
                        )
                    </div>
                </div>
            }.into(),
            display: view! {
                // https://www.sra.uni-hannover.de/news.html#news-2026-norcas-thomas
                // TODO List highlighting mit :target
                // Put #signatur in the url, show corresponding book
                <h1 class="h1">"Books display"</h1>
            }.into()
        )
    })
}
