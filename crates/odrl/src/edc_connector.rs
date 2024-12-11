// https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/0.5.0#/
// https://github.com/sovity/edc-extensions/tree/main/docs/deployment-guide/goals/local-demo

// api based on https://github.com/sovity/edc-extensions/blob/main/docs/api/eclipse-edc-management-api.yaml

use reqwest::Result;
extern crate edc_client as api;

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    let default_configuration = api::configuration::Configuration::default().with_headers();

    println!("\n\nGet all transfer processes\n");

    let query = api::QuerySpec::default();
    let response =
        api::transfer_process_api::query_transfer_processes(&default_configuration, Some(query))
            .await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }

    println!("\n\nGet transfer process by id\n");

    let transfer_process_id = "087bc937-3510-4550-b84a-ee4a62a7ad77";
    let response = api::transfer_process_api::get_transfer_process(
        &default_configuration,
        transfer_process_id,
    )
    .await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }

    println!("\n\nGet status of transfer process\n");

    let transfer_process_id = "087bc937-3510-4550-b84a-ee4a62a7ad77";
    let response = api::transfer_process_api::get_transfer_process_state(
        &default_configuration,
        transfer_process_id,
    )
    .await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }

    println!("\n\nInitiate transfer states\n");

    let mut data_destination = api::DataAddress::default();
    data_destination.r#type = Some("data-destination-type".to_string());
    let mut transfer_request = api::TransferRequest::default();
    transfer_request.asset_id = "testasset1".to_string();
    transfer_request.connector_address = Some("http://edc:11003/api/dsp".to_string());
    transfer_request.connector_id = Some("my-edc".to_string());
    transfer_request.contract_id =
        "dGVzdDI=:dGVzdGFzc2V0MQ==:Y2Q1ZTVkMGUtMTE0ZC00YjlmLWIxODAtM2RmYjYxMGQ5Y2Nk".to_string();
    transfer_request.data_destination = Box::new(data_destination);
    transfer_request.protocol = "dataspace-protocol-http".to_string();
    let response = api::transfer_process_api::initiate_transfer_process(
        &default_configuration,
        Some(transfer_request),
    )
    .await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }

    Ok(())
}
