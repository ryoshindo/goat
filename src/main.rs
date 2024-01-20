use aws_sdk_iam::error::SdkError;
use aws_sdk_iam::operation::list_access_keys::ListAccessKeysError;
use aws_sdk_iam::operation::list_users::ListUsersError;
use aws_sdk_iam::types::{AccessKeyMetadata, User};
use aws_sdk_iam::Client;

async fn list_users(client: &Client) -> Result<Vec<User>, SdkError<ListUsersError>> {
    let response = client.list_users().send().await?;

    Ok(response.users)
}

async fn list_access_keys_per_user(
    client: &Client,
    user_name: &str,
) -> Result<Vec<AccessKeyMetadata>, SdkError<ListAccessKeysError>> {
    let response = client
        .list_access_keys()
        .user_name(user_name)
        .send()
        .await?;

    Ok(response.access_key_metadata)
}

async fn list_access_keys(
    client: &Client,
    users: Vec<User>,
) -> Result<Vec<AccessKeyMetadata>, SdkError<ListAccessKeysError>> {
    let mut access_keys = Vec::new();
    for user in users {
        let response = list_access_keys_per_user(client, &user.user_name).await?;
        access_keys.extend(response);
    }

    Ok(access_keys)
}

#[tokio::main]
async fn main() {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let users = list_users(&client).await.expect("failed to list users");
    let access_keys = list_access_keys(&client, users)
        .await
        .expect("failed to list access keys");

    for access_key in access_keys {
        println!("access_key_id: {}", access_key.access_key_id.unwrap());
    }
}
