use google_cloud_bigquery::{
    client::{google_cloud_auth::credentials::CredentialsFile, Client, ClientConfig},
    http::{
        job::query::{QueryRequest, QueryResponse},
        table::{list::ListTablesRequest, TableReference},
        tabledata::list::FetchDataRequest,
    },
    storage::row::Row,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let json = include_str!("../../auth/biquery-integration-tests-70c0f9dad952.json");
    let credentials = CredentialsFile::new_from_str(&json).await?;

    let (config, project_id) = ClientConfig::new_with_credentials(credentials)
        .await
        .unwrap();
    let client = Client::new(config).await.unwrap();

    let project_id = project_id.expect("missing project_id");

    let QueryResponse { rows, schema, .. } = client
        .job()
        .query(
            &project_id,
            &QueryRequest {
                query: "SELECT * FROM beers.beers_tiny".to_string(),
                ..Default::default()
            },
        )
        .await?;

    for field in schema.iter().flat_map(|s| &s.fields) {
        println!("COL {:?}: {:?}", field.name, field.data_type);
    }

    for row in rows.iter().flatten() {
        println!("ROW");
        for cell in &row.f {
            println!("{:?}", cell.v);
        }
    }

    // Get arrow from table
    let result = client
        .tabledata()
        .read(
            &project_id,
            "beers",
            "beers_tiny",
            &FetchDataRequest {
                ..Default::default()
            },
        )
        .await?;

    println!("{:?}", result);

    let mut result = client
        .read_table::<Row>(
            &TableReference {
                project_id,
                dataset_id: "beers".to_string(),
                table_id: "beers_tiny".to_string(),
            },
            None,
        )
        .await?;

    while let Some(_row) = result.next().await? {
        println!("ROW");
    }

    Ok(())
}

async fn _notes() -> anyhow::Result<()> {
    let json = include_str!("../../auth/biquery-integration-tests-70c0f9dad952.json");
    let credentials = CredentialsFile::new_from_str(&json).await?;

    let (config, project_id) = ClientConfig::new_with_credentials(credentials)
        .await
        .unwrap();
    let client = Client::new(config).await.unwrap();

    let project_id = project_id.expect("missing project_id");

    // Fails when there is no table in result
    {
        println!(
            "{:#?}",
            client
                .table()
                .list(&project_id, "tpch_0_1", &ListTablesRequest::default())
                .await?
        );
        // Error: error decoding response body
        // Caused by:
        // missing field `tables` at line 5 column 1
    }

    Ok(())
}
