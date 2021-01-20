use super::shared::Backend;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rusoto_core;
use rusoto_credential::{
	AwsCredentials, CredentialsError, EnvironmentProvider, ProfileProvider, ProvideAwsCredentials,
};
use rusoto_s3::S3Client;
use rusoto_s3::S3 as RusotoS3;
use std::env;

pub struct S3 {}

// Provider types we support
#[derive(Debug)]
enum Provider {
	Environment(EnvironmentProvider),
	Profile(ProfileProvider),
}

// Implement ProvideAwsCredentials for our Provider enum
// Allowing us to pass our enum through to S3Client::new_with()
#[async_trait]
impl ProvideAwsCredentials for Provider {
	async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
		return self.credentials().await;
	}
}

impl S3 {
	/// Checks if all required env vars are provided
	/// Returns true on success and false on err
	fn has_env(&self) -> bool {
		return env::var("AWS_ACCESS_KEY_ID").is_ok() && env::var("AWS_SECRET_ACCESS_KEY").is_ok();
	}

	/// Returns true if the .aws/credentials file exists
	/// NOTE: Just because it exists does not mean it is valid/well formed
	fn has_credentials(&self) -> bool {
		let mut aws_creds = dirs::home_dir().unwrap();
		aws_creds.push(".aws");
		aws_creds.push("credentials");
		return aws_creds.exists();
	}

	fn get_client(&self, provider: Provider) -> Result<S3Client> {
		// TODO: Prompt user to confirm region is correct
		let region = rusoto_core::Region::default();
		let http = rusoto_core::HttpClient::new()?;

		// NOTE: Seems redundant but without this we get a core dump when calling list_buckets
		match provider {
			Provider::Environment(p) => Ok(S3Client::new_with(http, p, region)),
			Provider::Profile(p) => Ok(S3Client::new_with(http, p, region)),
		}
	}

	// NOTE: Can't get this working but hopefully will
	fn get_cred_provider(&self) -> Result<Provider> {
		if self.has_env() {
			return Ok(Provider::Environment(EnvironmentProvider::default()));
		} else if self.has_credentials() {
			return Ok(Provider::Profile(ProfileProvider::new().unwrap()));
		} else {
			return Err(anyhow!(
				"Failed to find aws credentials in ~/.aws/credentials or environement variables"
			));
		}
	}
}

#[async_trait]
impl Backend for S3 {
	fn new() -> S3 {
		S3 {}
	}

	async fn init(&self) -> Result<()> {
		let provider = self.get_cred_provider()?;
		let client = self.get_client(provider)?;

		let resp = client.list_buckets().await;

		if resp.is_ok() {
			return Ok(());
		}

		Err(anyhow!("Failed to validate AWS credentials"))
	}

	fn publish(&self) {}

	fn backup(&self) {}
}
