/*
 * Transfer Process [ACK]
 * Sent by:    Consumer, Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Transfer Process is an object returned by a Consumer or Provider indicating a successful state change happened.
 */

use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferProcess {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(
        rename = "dspace:state",
        deserialize_with = "deserialize_transfer_state_type"
    )]
    pub state: TransferStateType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferStateType {
    TransferState(TransferState),
    EDCTransferState(EDCTransferState),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransferState {
    #[serde(
        rename = "dspace:REQUESTED",
        alias = "https://w3id.org/dspace/v0.8/REQUESTED"
    )]
    REQUESTED,
    #[serde(
        rename = "dspace:STARTED",
        alias = "https://w3id.org/dspace/v0.8/STARTED"
    )]
    STARTED,
    #[serde(
        rename = "dspace:TERMINATED",
        alias = "https://w3id.org/dspace/v0.8/TERMINATED"
    )]
    TERMINATED,
    #[serde(
        rename = "dspace:COMPLETED",
        alias = "https://w3id.org/dspace/v0.8/COMPLETED"
    )]
    COMPLETED,
    #[serde(
        rename = "dspace:SUSPENDED",
        alias = "https://w3id.org/dspace/v0.8/SUSPENDED"
    )]
    SUSPENDED,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EDCTransferState {
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

// Custom deserialization for TransferStateType
fn deserialize_transfer_state_type<'de, D>(deserializer: D) -> Result<TransferStateType, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    match value {
        "dspace:REQUESTED" | "https://w3id.org/dspace/v0.8/REQUESTED" => {
            Ok(TransferStateType::TransferState(TransferState::REQUESTED))
        }
        "dspace:STARTED" | "https://w3id.org/dspace/v0.8/STARTED" => {
            Ok(TransferStateType::TransferState(TransferState::STARTED))
        }
        "dspace:TERMINATED" | "https://w3id.org/dspace/v0.8/TERMINATED" => {
            Ok(TransferStateType::TransferState(TransferState::TERMINATED))
        }
        "dspace:COMPLETED" | "https://w3id.org/dspace/v0.8/COMPLETED" => {
            Ok(TransferStateType::TransferState(TransferState::COMPLETED))
        }
        "dspace:SUSPENDED" | "https://w3id.org/dspace/v0.8/SUSPENDED" => {
            Ok(TransferStateType::TransferState(TransferState::SUSPENDED))
        }
        "INITIAL" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Initial,
        )),
        "PROVISIONING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Provisioning,
        )),
        "PROVISIONING_REQUESTED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::ProvisioningRequested,
        )),
        "PROVISIONED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Provisioned,
        )),
        "REQUESTING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Requesting,
        )),
        "REQUESTED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Requested,
        )),
        "STARTING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Starting,
        )),
        "STARTED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Started,
        )),
        "SUSPENDING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Suspending,
        )),
        "SUSPENDED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Suspended,
        )),
        "RESUMING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Resuming,
        )),
        "RESUMED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Resumed,
        )),
        "COMPLETING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Completing,
        )),
        "COMPLETED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Completed,
        )),
        "TERMINATING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Terminating,
        )),
        "TERMINATED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Terminated,
        )),
        "DEPROVISIONING" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Deprovisioning,
        )),
        "DEPROVISIONING_REQUESTED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::DeprovisioningRequested,
        )),
        "DEPROVISIONED" => Ok(TransferStateType::EDCTransferState(
            EDCTransferState::Deprovisioned,
        )),
        _ => Err(de::Error::invalid_value(
            Unexpected::Str(value),
            &"a valid state",
        )),
    }
}

impl TransferProcess {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        provider_pid: String,
        consumer_pid: String,
        state: TransferStateType,
    ) -> TransferProcess {
        TransferProcess {
            context,
            dsp_type,
            provider_pid,
            consumer_pid,
            state,
        }
    }
}
