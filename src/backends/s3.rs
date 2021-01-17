use super::shared::Backend;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rusoto_core;
use rusoto_credential::{EnvironmentProvider, ProfileProvider, ProvideAwsCredentials};
use rusoto_s3::S3Client;
use std::env;

pub struct S3 {}

impl S3 {
	/// Checks if all required env vars are provided
	/// Returns true on success and false on err
	fn has_env(&self) -> bool {
		let required = vec!["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY"];

		for key in required {
			if let Err(_) = env::var(key) {
				return false;
			}
		}

		true
	}

	/// Returns true if the .aws/credentials file exists
	/// NOTE: Just because it exists does not mean it is valid/well formed
	fn has_credentials(&self) -> bool {
		let mut aws_creds = dirs::home_dir().unwrap();
		aws_creds.push(".aws");
		aws_creds.push("credentials");
		return aws_creds.exists();
	}

	fn get_client<P>(&self, provider: P) -> Result<S3Client>
	where
		P: ProvideAwsCredentials + Sync + Send + 'static,
	{
		// TODO: Prompt user to confirm region is correct
		let region = rusoto_core::Region::default();
		let http = rusoto_core::HttpClient::new().unwrap();
		Ok(S3Client::new_with(http, provider, region))
	}
}

#[async_trait]
impl Backend for S3 {
	fn new() -> S3 {
		S3 {}
	}

	fn init(&self) -> Result<()> {
		// TODO: Tidy this up
		// TODO: Test credentials actually work
		if self.has_env() {
			self.get_client(EnvironmentProvider::default())?;
		} else if self.has_credentials() {
			self.get_client(ProfileProvider::new().unwrap())?;
		} else {
			return Err(anyhow!(
				"Failed to find aws credentials in ~/.aws/credentials or environement variables"
			));
		};

		Ok(())
	}

	fn publish(&self) {}

	fn backup(&self) {}
}
