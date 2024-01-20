use aws_sdk_iam::error::SdkError;
use aws_sdk_iam::operation::list_access_keys::{ListAccessKeysError, ListAccessKeysOutput};
use aws_sdk_iam::operation::list_users::{ListUsersError, ListUsersOutput};
use aws_sdk_iam::Client;

async fn list_users(client: &Client) -> Result<ListUsersOutput, SdkError<ListUsersError>> {
    let response = client.list_users().send().await?;

    Ok(response)
}

async fn list_access_keys_per_user(
    client: &Client,
    user_name: &str,
) -> Result<ListAccessKeysOutput, SdkError<ListAccessKeysError>> {
    let response = client
        .list_access_keys()
        .user_name(user_name)
        .send()
        .await?;

    for access_key_metadata in response.clone().access_key_metadata {
        println!(
            "{}: {}",
            user_name,
            access_key_metadata.access_key_id.unwrap()
        );
    }

    Ok(response)
}

#[tokio::main]
async fn main() {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let users = list_users(&client).await.expect("failed to list users");
    for user in users.users {
        list_access_keys_per_user(&client, &user.user_name)
            .await
            .expect("failed to list access keys");
    }
}
