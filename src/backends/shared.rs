use anyhow::Result;
/// Shared backend logic (traits)
use async_trait::async_trait;

#[async_trait]
pub trait Backend {
	fn new() -> Self;

	/// Initialise new backend
	/// Gather keys etc...
	async fn init(&self) -> Result<()>;

	/// Publish blog posts
	fn publish(&self);

	/// Create a local zip of all blog posts
	fn backup(&self);
}
