use aws_sdk_dynamodb::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let resp = client
        .list_tables()
        .limit(10)
        .send()
        .await?;

    println!("Tables: {:?}", resp.table_names.unwrap());

    Ok(())
}
