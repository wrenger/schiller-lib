use topcoat::{
    Result,
    context::Cx,
    router::{error::redirect_permanent, page},
};

#[page]
pub async fn page(cx: &Cx) -> Result<()> {
    crate::server::require_login(cx).await?;
    Err(redirect_permanent("/books").into())
}
