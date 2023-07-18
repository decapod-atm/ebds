#[cfg(not(feature = "std"))]
use alloc::string::String;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::std;
use std::fmt;

use crate::{
    cash::{BillAcceptorConfig, CashInsertionEvent, DispenseRequest},
    error::{JsonRpcError, JsonRpcResult as Result},
    hardware::HardwareStatus,
    method::Method,
};

pub const JRPC_VERSION: &str = "2.0";
pub const OPEN_BRACE: &str = "{";
pub const CLOSE_BRACE: &str = "}";

/// HAL payload types
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HalPayload {
    Empty(()),
    Error(JsonRpcError),
    CashInsertionEvent(CashInsertionEvent),
    DispenseRequest(DispenseRequest),
    BillAcceptorConfig(BillAcceptorConfig),
    HardwareStatus(HardwareStatus),
}

impl Default for HalPayload {
    fn default() -> Self {
        Self::Empty(())
    }
}

impl fmt::Display for HalPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(()) => write!(f, "()"),
            Self::Error(inner) => write!(f, "{inner}"),
            Self::CashInsertionEvent(inner) => write!(f, "{inner}"),
            Self::DispenseRequest(inner) => write!(f, "{inner}"),
            Self::BillAcceptorConfig(inner) => write!(f, "{inner}"),
            Self::HardwareStatus(inner) => write!(f, "{inner}"),
        }
    }
}

/// JSON-RPC ID
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonRpcId {
    Integer(u64),
    String(String),
}

impl fmt::Display for JsonRpcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(id) => write!(f, "{id}"),
            Self::String(id) => write!(f, "{id}"),
        }
    }
}

/// JSON-RPC message carrying a HAL payload
#[repr(C)]
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcMessage {
    /// JSON-RPC version string (should always be "2.0")
    jsonrpc: String,
    /// JSON-RPC ID (typically 1)
    id: Option<JsonRpcId>,
    /// JSON-RPC method being called/responded to
    method: Option<Method>,
    /// HAL message payload
    data: Option<HalPayload>,
}

impl JsonRpcMessage {
    /// Create a new JsonRpcMessage with the provided method and payload
    pub fn new(method: Method, data: HalPayload) -> Self {
        Self {
            jsonrpc: JRPC_VERSION.into(),
            id: Some(JsonRpcId::Integer(1)),
            method: Some(method),
            data: Some(data),
        }
    }

    /// Create a JsonRpcMessage from a method and payload
    ///
    /// Validates the method-payload pair, and returns the JsonRpcMessage
    /// Returns error for an invalid method-payload pair
    pub fn create(method: Method, payload: HalPayload) -> Result<Self> {
        let msg = match (method, &payload) {
            (Method::Accept, HalPayload::BillAcceptorConfig(_))
            | (Method::Stop, _)
            | (Method::Dispense, _)
            | (Method::Reject, HalPayload::CashInsertionEvent(_))
            | (Method::Stack, HalPayload::CashInsertionEvent(_))
            | (Method::Status, HalPayload::HardwareStatus(_))
            | (_, HalPayload::Empty(_)) => Self::new(method, payload),
            _ => return Err(JsonRpcError::failure("invalid method-payload combination")),
        };

        Ok(msg)
    }

    pub fn jsonrpc(&self) -> &str {
        self.jsonrpc.as_str()
    }

    pub fn id(&self) -> u64 {
        if let Some(&JsonRpcId::Integer(id)) = self.id.as_ref() {
            id
        } else {
            1
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(JsonRpcId::Integer(id));
    }

    /// Get the JSON-RPC method
    pub fn method(&self) -> Method {
        self.method.unwrap_or(Method::default())
    }

    /// Get the JSON-RPC data (HalPayload)
    pub fn data(&self) -> Option<&HalPayload> {
        self.data.as_ref()
    }
}

impl Default for JsonRpcMessage {
    fn default() -> Self {
        Self::new(Method::default(), HalPayload::default())
    }
}

impl fmt::Display for JsonRpcMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let jsonrpc = self.jsonrpc();
        let id = match self.id.as_ref() {
            Some(JsonRpcId::Integer(id)) => format!("{}", id),
            Some(JsonRpcId::String(id)) => id.clone(),
            None => "1".into(),
        };
        let method = if let Some(method) = self.method.as_ref() {
            format!("\"method\":\"{method}\",")
        } else {
            String::new()
        };
        let data = if let Some(data) = self.data.as_ref() {
            format!("\"data\":{data}")
        } else {
            String::new()
        };
        write!(
            f,
            "{OPEN_BRACE}\"json-rpc\":\"{jsonrpc}\", \"id\":\"{id}\",{method} {data}{CLOSE_BRACE}",
        )
    }
}

impl From<JsonRpcRequest> for JsonRpcMessage {
    fn from(msg: JsonRpcRequest) -> Self {
        Self {
            jsonrpc: msg.jsonrpc,
            id: msg.id,
            method: msg.method,
            data: msg.params,
        }
    }
}

impl From<JsonRpcResponse> for JsonRpcMessage {
    fn from(msg: JsonRpcResponse) -> Self {
        Self {
            jsonrpc: msg.jsonrpc,
            id: msg.id,
            method: None,
            data: msg.result,
        }
    }
}

/// JSON-RPC message carrying a HAL payload
#[repr(C)]
#[derive(Clone, Debug, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version string (should always be "2.0")
    jsonrpc: String,
    /// JSON-RPC ID (typically 1)
    id: Option<JsonRpcId>,
    /// JSON-RPC method being called/responded to
    method: Option<Method>,
    /// HAL message payload
    params: Option<HalPayload>,
}

impl JsonRpcRequest {
    /// Create a new JsonRpcMessage with the provided method and payload
    pub fn new(method: Method, params: HalPayload) -> Self {
        Self {
            jsonrpc: JRPC_VERSION.into(),
            id: Some(JsonRpcId::Integer(1)),
            method: Some(method),
            params: Some(params),
        }
    }

    pub fn new_with_no_id(method: Method, params: HalPayload) -> Self {
        Self {
            jsonrpc: JRPC_VERSION.into(),
            id: None,
            method: Some(method),
            params: Some(params),
        }
    }

    pub fn jsonrpc(&self) -> &str {
        self.jsonrpc.as_str()
    }

    /// Gets the JSON-RPC ID value.
    pub fn id(&self) -> u64 {
        if let Some(&JsonRpcId::Integer(id)) = self.id.as_ref() {
            id
        } else {
            1
        }
    }

    /// Sets the JSON-RPC ID value.
    pub fn set_id(&mut self, id: u64) {
        self.id = Some(JsonRpcId::Integer(id));
    }

    /// Unsets the JSON-RPC ID value.
    pub fn unset_id(&mut self) -> Option<JsonRpcId> {
        self.id.take()
    }

    /// Get the JSON-RPC method
    pub fn method(&self) -> Method {
        self.method.unwrap_or(Method::default())
    }

    /// Get the JSON-RPC request parameters (HalPayload)
    pub fn params(&self) -> Option<&HalPayload> {
        self.params.as_ref()
    }
}

impl Default for JsonRpcRequest {
    fn default() -> Self {
        Self::new(Method::default(), HalPayload::default())
    }
}

impl fmt::Display for JsonRpcRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let jsonrpc = self.jsonrpc();
        let id = if let Some(id) = self.id.as_ref() {
            format!(", \"id\":\"{id}\"")
        } else {
            String::new()
        };
        let method = if let Some(method) = self.method.as_ref() {
            format!(", \"method\":\"{method}\"")
        } else {
            String::new()
        };
        let params = if let Some(params) = self.params.as_ref() {
            format!(", \"params\":{params}")
        } else {
            String::new()
        };
        write!(
            f,
            "{OPEN_BRACE}\"json-rpc\":\"{jsonrpc}\"{id}{method}{params}{CLOSE_BRACE}",
        )
    }
}

impl Serialize for JsonRpcRequest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut request = serializer.serialize_struct("JsonRpcRequest", 4)?;

        request.serialize_field("jsonrpc", &self.jsonrpc)?;

        if let Some(id) = self.id.as_ref() {
            request.serialize_field("id", id)?;
        }
        if let Some(method) = self.method.as_ref() {
            request.serialize_field("method", method)?;
        }
        if let Some(params) = self.params.as_ref() {
            request.serialize_field("params", params)?;
        }

        request.end()
    }
}

/// JSON-RPC message carrying a HAL payload
#[repr(C)]
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version string (should always be "2.0")
    jsonrpc: String,
    /// JSON-RPC ID (typically 1)
    id: Option<JsonRpcId>,
    /// HAL message payload
    result: Option<HalPayload>,
}

impl JsonRpcResponse {
    /// Create a new JsonRpcMessage with the provided payload
    pub fn new(result: HalPayload) -> Self {
        Self {
            jsonrpc: JRPC_VERSION.into(),
            id: Some(JsonRpcId::Integer(1)),
            result: Some(result),
        }
    }

    /// Create a new JsonRpcMessage with the provided ID and payload
    pub fn new_with_id(id: u64, result: HalPayload) -> Self {
        Self {
            jsonrpc: JRPC_VERSION.into(),
            id: Some(JsonRpcId::Integer(id)),
            result: Some(result),
        }
    }

    /// Create a new JsonRpcMessage with the provided ID and no payload
    pub fn new_with_id_only(id: u64) -> Self {
        Self {
            jsonrpc: JRPC_VERSION.into(),
            id: Some(JsonRpcId::Integer(id)),
            result: None,
        }
    }

    pub fn jsonrpc(&self) -> &str {
        self.jsonrpc.as_str()
    }

    pub fn id(&self) -> u64 {
        if let Some(&JsonRpcId::Integer(id)) = self.id.as_ref() {
            id
        } else {
            1
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(JsonRpcId::Integer(id));
    }

    /// Get the JSON-RPC response result (HalPayload)
    pub fn result(&self) -> Option<&HalPayload> {
        self.result.as_ref()
    }
}

impl Default for JsonRpcResponse {
    fn default() -> Self {
        Self::new(HalPayload::default())
    }
}

impl fmt::Display for JsonRpcResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let jsonrpc = self.jsonrpc();
        let id = match self.id.as_ref() {
            Some(JsonRpcId::Integer(id)) => format!("{}", id),
            Some(JsonRpcId::String(id)) => id.clone(),
            None => "1".into(),
        };
        let result = if let Some(result) = self.result.as_ref() {
            format!(", \"result\":{result}")
        } else {
            String::new()
        };
        write!(
            f,
            "{OPEN_BRACE}\"json-rpc\":\"{jsonrpc}\", \"id\":\"{id}\"{result}{CLOSE_BRACE}",
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;
    use crate::{
        cash::Currency,
        error::JsonRpcError,
        hardware::{BillAcceptorStatusDetails, HardwareComponent, HardwareState},
    };
    use serde_json::{self, Result};

    #[test]
    fn test_json_rpc_id_serde() -> Result<()> {
        let id = JsonRpcId::Integer(42);
        let expected = "42";

        assert_eq!(serde_json::to_string(&id)?, expected);
        assert_eq!(serde_json::from_str::<JsonRpcId>(expected)?, id);

        Ok(())
    }

    #[test]
    fn test_json_rpc_hal_payload_serde() -> Result<()> {
        let payload = HalPayload::Error(JsonRpcError::new(-32700, "Parse error"));
        let expected = "{\"code\":{\"GenericError\":-32700},\"message\":\"Parse error\"}";

        assert_eq!(serde_json::to_string(&payload)?, expected);
        assert_eq!(serde_json::from_str::<HalPayload>(expected)?, payload);

        #[cfg(feature = "std")]
        println!(
            "{}",
            serde_json::to_string(&JsonRpcRequest::new(Method::Unknown, payload))?
        );

        let payload = HalPayload::CashInsertionEvent(CashInsertionEvent::new(Method::Accept, 42));
        let expected = "{\"event\":\"ACCEPT\",\"amount\":42}";

        assert_eq!(serde_json::to_string(&payload)?, expected);
        assert_eq!(serde_json::from_str::<HalPayload>(expected)?, payload);

        #[cfg(feature = "std")]
        println!(
            "{}",
            serde_json::to_string(&JsonRpcRequest::new(Method::Stack, payload))?
        );

        let payload = HalPayload::CashInsertionEvent(CashInsertionEvent::new(Method::Reject, 42));
        let expected = "{\"event\":\"REJECT\",\"amount\":42}";

        assert_eq!(serde_json::to_string(&payload)?, expected);
        assert_eq!(serde_json::from_str::<HalPayload>(expected)?, payload);

        #[cfg(feature = "std")]
        println!(
            "{}",
            serde_json::to_string(&JsonRpcRequest::new(Method::Reject, payload))?
        );

        let payload = HalPayload::DispenseRequest(DispenseRequest::new(42, Currency::USD));
        let expected = "{\"amount\":42,\"currency\":\"USD\"}";

        assert_eq!(serde_json::to_string(&payload)?, expected);
        assert_eq!(serde_json::from_str::<HalPayload>(expected)?, payload);

        #[cfg(feature = "std")]
        println!(
            "{}",
            serde_json::to_string(&JsonRpcRequest::new(Method::Dispense, payload))?
        );

        let payload = HalPayload::BillAcceptorConfig(BillAcceptorConfig::new(Currency::USD));
        let expected = "{\"currency\":\"USD\"}";

        assert_eq!(serde_json::to_string(&payload)?, expected);
        assert_eq!(serde_json::from_str::<HalPayload>(expected)?, payload);

        #[cfg(feature = "std")]
        println!(
            "{}",
            serde_json::to_string(&JsonRpcRequest::new(Method::Accept, payload))?
        );

        let payload = HalPayload::HardwareStatus(HardwareStatus::new(
            HardwareComponent::BAU,
            HardwareState::OK,
            "BAU is operating properly".to_string(),
            BillAcceptorStatusDetails::default().with_firmware_version("1.0"),
        ));
        let expected = "{\"component\":\"BAU\",\"state\":\"OK\",\"description\":\"BAU is operating properly\",\"details\":{\"cashbox_removed\":null,\"firmware_version\":\"1.0\",\"currency\":null,\"jammed\":null}}";

        assert_eq!(serde_json::to_string(&payload)?, expected);
        assert_eq!(serde_json::from_str::<HalPayload>(expected)?, payload);

        #[cfg(feature = "std")]
        println!(
            "{}",
            serde_json::to_string(&JsonRpcRequest::new(Method::Status, payload))?
        );

        Ok(())
    }
}
