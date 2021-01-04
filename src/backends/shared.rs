/// Shared backend logic (traits)

pub trait Backend {
	/// Initialise new backend
	/// Gather keys etc...
	fn init(&self);

	/// Publish blog posts
	fn publish(&self);

	/// Create a local zip of all blog posts
	fn backup(&self);
}
