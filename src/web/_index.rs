use topcoat::{
    Result,
    router::{error::redirect_permanent, page},
};

#[page]
pub async fn page() -> Result<()> {
    Err(redirect_permanent("/books").into())
}
