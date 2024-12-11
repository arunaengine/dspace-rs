use crate::utils::remove_prefixes_from_value;
use crate::ResponseContent;
use crate::{configuration, Error};
use reqwest;
use reqwest::header::AUTHORIZATION;

/// struct for typed errors of method [`get_dspace_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDspaceVersionError {
    UnknownValue(serde_json::Value),
}

/// The contents of the response is a JSON object defined in the Dataspace Protocol.
pub async fn get_dspace_version(
    configuration: &configuration::Configuration,
) -> Result<dsp_api::common::DspaceVersion, Error<GetDspaceVersionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/.well-known/dspace-version",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.header(
        AUTHORIZATION,
        "{\"region\": \"any\", \"audience\": \"any\", \"clientId\": \"any\"}",
    );

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDspaceVersionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
