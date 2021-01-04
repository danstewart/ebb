use super::shared::Backend;

struct S3 {

}

impl Backend for S3 {
	fn init(&self) {}

	fn publish(&self) {}

	fn backup(&self) {}
}
