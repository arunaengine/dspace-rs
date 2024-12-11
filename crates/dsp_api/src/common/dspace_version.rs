/*
 * Connectors implementing the Dataspace Protocol may operate on different versions.
 * Therefore, it is necessary that they can discover the supported versions of each other reliably and unambiguously.
 * Each Connector must expose information of at least one Dataspace Protocol Version it supports.
 * The specifics of how this information is obtained its defined by specific protocol bindings.
 *
 * A Connector must respond to a respective request by providing a JSON-LD object containing an array of supported versions with at least one item.
 * The item connects the version tag (version attribute) with the absolute URL path segment of the root path for all endpoints of this version.
 * The following example specifies that this Connector offers version 1.0 endpoints at <host>/some/path/v1.
 *
 * The requesting Connector may select from the endpoints in the response.
 * If the Connector can't identify a matching Dataspace Protocol Version, it must terminate the communication.
 */

use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DspaceVersion {
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "protocolVersions")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub protocol_version: Vec<ProtocolVersion>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolVersion {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "path")]
    pub path: String,
}

impl DspaceVersion {
    pub fn new(
        context: std::collections::HashMap<String, serde_json::Value>,
        protocol_version: Vec<ProtocolVersion>,
    ) -> DspaceVersion {
        DspaceVersion {
            context,
            protocol_version,
        }
    }
}

impl ProtocolVersion {
    pub fn new(version: String, path: String) -> ProtocolVersion {
        ProtocolVersion { version, path }
    }
}
