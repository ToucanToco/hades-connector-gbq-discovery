use gcp_bigquery_client::{
    model::{
        query_request::QueryRequest, query_response::QueryResponse,
        table_field_schema::TableFieldSchema,
    },
    tabledata::ListQueryParameters,
};

const PROJECT_ID: &str = "biquery-integration-tests";
const DATASET_ID: &str = "beers";
const TABLE_ID: &str = "beers_tiny";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init BigQuery client
    let client = gcp_bigquery_client::Client::from_service_account_key_file(
        "../auth/biquery-integration-tests-70c0f9dad952.json",
    )
    .await?;

    let QueryResponse { schema, rows, .. } = client
        .job()
        .query(
            PROJECT_ID,
            QueryRequest {
                query: "SELECT * FROM beers.beers_tiny".to_string(),
                ..Default::default()
            },
        )
        .await?;

    let fields: Vec<TableFieldSchema> = schema
        .map(|s| s.fields)
        .into_iter()
        .flatten()
        .flatten()
        .collect();

    for field in fields {
        println!("COL {:?}: {:?}", field.name, field.r#type);
    }

    for row in rows.iter().flatten() {
        println!("ROW");
        for cell in row.columns.iter().flatten() {
            println!("{:?}", cell.value);
        }
    }

    // Get arrow from table
    let result = client
        .tabledata()
        .list(
            PROJECT_ID,
            DATASET_ID,
            TABLE_ID,
            ListQueryParameters {
                start_index: None,
                max_results: None,
                page_token: None,
                selected_fields: None,
                format_options: None,
            },
        )
        .await?;

    println!("{result:?}");

    Ok(())
}
