pub mod _index;
pub mod books;
pub mod info;
pub mod overdues;
pub mod settings;
pub mod users;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    font::fontsource::fontsource_font,
    router::{RouterBuilder, RouterBuilderDiscoverExt, Slot, layout, module_router},
    runtime::RouterBuilderRuntimeExt,
    session::{RouterBuilderSessionExt, SessionConfig},
    tailwind,
    view::{View, view},
};

use crate::db::AtomicDatabase;
use crate::util::FAVICON;

pub fn router(db: AtomicDatabase) -> RouterBuilder {
    module_router!()
        .app_context(db)
        .discover()
        .assets(AssetBundle::load().unwrap())
        .sessions(SessionConfig::default())
        .runtime()
}

#[layout]
async fn root(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html class="dark">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Hello world"</title>
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
