use crate::t;

pub mod _index;
pub mod books;
pub mod info;
pub mod overdues;
pub mod settings;
pub mod users;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    font::{RouterBuilderFontExt, fontsource::fontsource_font},
    l10n::{accept_language, locale, negotiate},
    router::{Body, Next, RouterBuilder, Slot, layer, layout, module_router, response::Response},
    tailwind,
    view::{View, view},
};

use crate::db::AtomicDatabase;
use crate::util::FAVICON;

pub fn router(db: AtomicDatabase) -> RouterBuilder {
    module_router!()
        .app_context(db)
        .discover_fonts()
        .assets(AssetBundle::load().unwrap())
}

#[layer]
async fn language(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let selected = negotiate(accept_language(cx), crate::t::locales())
        .cloned()
        .unwrap_or(locale!("en"));
    next.run(&cx.with(selected), body).await
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html class="dark" lang=(locale(cx).id()) dir=(locale(cx).id().direction())>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>(t!(main_title))</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                <link rel="icon" href=(FAVICON) />
                topcoat::font::link(font: fontsource_font!(GEIST))
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>

            (slot)
        </html>
    })
}
