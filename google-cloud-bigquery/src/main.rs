use google_cloud_bigquery::client::{google_cloud_auth::credentials::{CredentialsFile}, Client, ClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let json = include_str!("../../auth/biquery-integration-tests-70c0f9dad952.json");
    let credentials = CredentialsFile::new_from_str(&json).await?;

    let (config, project_id) = ClientConfig::new_with_credentials(credentials).await.unwrap();
    let client = Client::new(config).await.unwrap();

    let project_id = project_id.expect("missing project_id");

    let result = client.dataset().list(&project_id, None).await?;

    println!("{result:#?}");

    Ok(())
}