use gcp_bigquery_client::dataset::ListOptions;


const PROJECT_ID: &str = "biquery-integration-tests";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init BigQuery client
    let client = gcp_bigquery_client::Client::from_service_account_key_file("../auth/biquery-integration-tests-70c0f9dad952.json").await?;
    
    let result = client.dataset().list(PROJECT_ID, ListOptions::default()).await?;

    println!("{result:#?}");

    // Query
    // let mut query_response = client
    //     .job()
    //     .query(
    //         project_id,
    //         QueryRequest::new(format!(
    //             "SELECT COUNT(*) AS c FROM `{}.{}.{}`",
    //             project_id, dataset_id, table_id
    //         )),
    //     )
    //     .await?;
    // let mut rs = ResultSet::new_from_query_response(query_response);
    // while rs.next_row() {
    //     println!("Number of rows inserted: {}", rs.get_i64_by_name("c")?.unwrap());
    // }

    Ok(())
}
