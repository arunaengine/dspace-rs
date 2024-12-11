/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */
use utoipa::ToSchema;

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema,
)]
pub struct TransferState {
    #[serde(rename = "state")]
    pub state: TransferProcessState,
}

/// https://eclipse-edc.github.io/docs/#/submodule/Connector/docs/developer/transfer-process
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, ToSchema,
)]
pub enum TransferProcessState {
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "PROVISIONING")]
    Provisioning,
    #[serde(rename = "PROVISIONING_REQUESTED")]
    ProvisioningRequested,
    #[serde(rename = "PROVISIONED")]
    Provisioned,
    #[serde(rename = "REQUESTING")]
    Requesting,
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "SUSPENDING")]
    Suspending,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "RESUMING")]
    Resuming,
    #[serde(rename = "RESUMED")]
    Resumed,
    #[serde(rename = "COMPLETING")]
    Completing,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "TERMINATING")]
    Terminating,
    #[serde(rename = "TERMINATED")]
    Terminated,
    #[serde(rename = "DEPROVISIONING")]
    Deprovisioning,
    #[serde(rename = "DEPROVISIONING_REQUESTED")]
    DeprovisioningRequested,
    #[serde(rename = "DEPROVISIONED")]
    Deprovisioned,
}

impl Default for TransferState {
    fn default() -> Self {
        Self {
            state: TransferProcessState::Initial,
        }
    }
}
