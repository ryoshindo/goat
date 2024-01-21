use aws_sdk_iam::error::SdkError;
use aws_sdk_iam::operation::list_access_keys::ListAccessKeysError;
use aws_sdk_iam::operation::list_users::ListUsersError;
use aws_sdk_iam::types::{AccessKeyMetadata, User};
use aws_sdk_iam::Client;

struct Command {
    client: Client,
}

impl Command {
    fn new(client: Client) -> Self {
        Self { client }
    }

    async fn list_users(&self) -> Result<Vec<User>, SdkError<ListUsersError>> {
        let response = self.client.list_users().send().await?;

        Ok(response.users)
    }

    async fn list_access_keys_per_user(
        &self,
        user_name: &str,
    ) -> Result<Vec<AccessKeyMetadata>, SdkError<ListAccessKeysError>> {
        let response = self
            .client
            .list_access_keys()
            .user_name(user_name)
            .send()
            .await?;

        Ok(response.access_key_metadata)
    }

    async fn list_access_keys(
        &self,
    ) -> Result<Vec<AccessKeyMetadata>, SdkError<ListAccessKeysError>> {
        let mut access_keys = Vec::new();
        for user in self.list_users().await.unwrap() {
            let mut access_keys_per_user = self
                .list_access_keys_per_user(&user.user_name)
                .await
                .unwrap();
            access_keys.append(&mut access_keys_per_user);
        }

        Ok(access_keys)
    }
}

#[tokio::main]
async fn main() {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let command = Command::new(client);

    let access_keys = command.list_access_keys().await.unwrap();

    for access_key in access_keys {
        println!("access_key_id: {}", access_key.access_key_id.unwrap());
    }
}
