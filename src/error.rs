#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::std;
use std::{fmt, result};

use serde::{Deserialize, Serialize};

pub type Result<T> = result::Result<T, Error>;
pub type JsonRpcResult<T> = result::Result<T, JsonRpcError>;

/// Result status for HAL function calls
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum HalResult {
    Success = 0,
    Failure = 1,
    LogicFailure = 2,
    RuntimeFailure = 3,
    InvalidArgumentFailure = 4,
    OutOfRangeFailure = 5,
    NakResponse = 6,
    IllegalCall = 7,
    AsyncInterrupted = 8,
    InsufficientException = 9,
    TimeOut = 10,
    ErrorInvalidArguments = 4105,
    InvalidHandle = 4112,
    IncompleteArguments = 4113,
}

impl From<u16> for HalResult {
    fn from(res: u16) -> Self {
        match res {
            0 => Self::Success,
            1 => Self::Failure,
            2 => Self::LogicFailure,
            3 => Self::RuntimeFailure,
            4 => Self::InvalidArgumentFailure,
            5 => Self::OutOfRangeFailure,
            6 => Self::NakResponse,
            7 => Self::IllegalCall,
            8 => Self::AsyncInterrupted,
            9 => Self::InsufficientException,
            10 => Self::TimeOut,
            4105 => Self::ErrorInvalidArguments,
            4112 => Self::InvalidHandle,
            4113 => Self::IncompleteArguments,
            _ => Self::Failure,
        }
    }
}

impl From<HalResult> for &'static str {
    fn from(res: HalResult) -> Self {
        match res {
            HalResult::Success => "success",
            HalResult::Failure => "failure",
            HalResult::LogicFailure => "logic failure",
            HalResult::RuntimeFailure => "runtime failure",
            HalResult::InvalidArgumentFailure => "invalid argument failure",
            HalResult::OutOfRangeFailure => "out of range failure",
            HalResult::NakResponse => "NAK response",
            HalResult::IllegalCall => "illegal call",
            HalResult::AsyncInterrupted => "async interrupted",
            HalResult::InsufficientException => "insufficient exception",
            HalResult::TimeOut => "time out",
            HalResult::ErrorInvalidArguments => "error invalid arguments",
            HalResult::InvalidHandle => "invalid handle",
            HalResult::IncompleteArguments => "incomplete arguments",
        }
    }
}

impl From<&HalResult> for &'static str {
    fn from(res: &HalResult) -> &'static str {
        (*res).into()
    }
}

impl fmt::Display for HalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = <&'static str>::from(self);
        let code = (*self) as u16;

        write!(f, "{s}: {code}")
    }
}

/// HAL error codes
#[repr(i16)]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum HalError {
    LicenseKeyAuthFailure = -18,
    BillPresentInEscrow = -15,
    BillNotPresentInEscrow = -14,
    BillReject = -13,
    PrintImageErr = -12,
    OpenSerialPortFailure = -11,
    BufferOverflow = -10,
    DeviceTimeout = -9,
    LibraryInternalErr = -8,
    DeviceAlreadyOpen = -7,
    DeviceNotReady = -6,
    Cancelled = -5,
    InvalidData = -3,
    DeviceBusy = -2,
    DeviceFailure = -1,
    RequestFieldInvalid = 1,
    InternalErr = 2,
    InternalValidationErr = 3,
    ComponentNotImplemented = 4,
    PreconditionFailed = 5,
    ApplicationTimeout = 6,
    InvalidDenomination = 7,
    ApplicationBusy = 8,
    DeviceCommErr = 9,
    FirmwareErr = 10,
    PhysicalTamper = 11,
    SystemErr = 12,
    MethodNotImplemented = 13,
    DecodingErr = 14,
}

impl From<HalError> for &'static str {
    fn from(err: HalError) -> Self {
        match err {
            HalError::LicenseKeyAuthFailure => "license key auth failure",
            HalError::BillPresentInEscrow => "bill present in escrow",
            HalError::BillNotPresentInEscrow => "bill not present in escrow",
            HalError::BillReject => "bill reject",
            HalError::PrintImageErr => "print image err",
            HalError::OpenSerialPortFailure => "open serial port failure",
            HalError::BufferOverflow => "buffer overflow",
            HalError::DeviceTimeout => "device timeout",
            HalError::LibraryInternalErr => "library internal err",
            HalError::DeviceAlreadyOpen => "device already open",
            HalError::DeviceNotReady => "device not ready",
            HalError::Cancelled => "cancelled",
            HalError::InvalidData => "invalid data",
            HalError::DeviceBusy => "device busy",
            HalError::DeviceFailure => "device failure",
            HalError::RequestFieldInvalid => "request field invalid",
            HalError::InternalErr => "internal err",
            HalError::InternalValidationErr => "internal validation err",
            HalError::ComponentNotImplemented => "component not implemented",
            HalError::PreconditionFailed => "precondition failed",
            HalError::ApplicationTimeout => "application timeout",
            HalError::InvalidDenomination => "invalid denomination",
            HalError::ApplicationBusy => "application busy",
            HalError::DeviceCommErr => "device comm err",
            HalError::FirmwareErr => "firmware err",
            HalError::PhysicalTamper => "physical tamper",
            HalError::SystemErr => "system err",
            HalError::MethodNotImplemented => "method not implemented",
            HalError::DecodingErr => "decoding err",
        }
    }
}

impl From<&HalError> for &'static str {
    fn from(err: &HalError) -> Self {
        (*err).into()
    }
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &'static str = self.into();
        let code = (*self) as i32;

        write!(f, "{s}: {code}")
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum JsonRpcErrorCode {
    HalResult(HalResult),
    HalError(HalError),
    IoError(String),
    JsonError(String),
    SerialError(String),
    GenericError(i64),
    Stop,
}

impl From<HalResult> for JsonRpcErrorCode {
    fn from(err: HalResult) -> Self {
        Self::HalResult(err)
    }
}

impl From<HalError> for JsonRpcErrorCode {
    fn from(err: HalError) -> Self {
        Self::HalError(err)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for JsonRpcErrorCode {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(format!("{err}"))
    }
}

impl From<serde_json::Error> for JsonRpcErrorCode {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(format!("{err}"))
    }
}

impl From<i64> for JsonRpcErrorCode {
    fn from(err: i64) -> Self {
        Self::GenericError(err)
    }
}

impl fmt::Display for JsonRpcErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HalResult(err) => write!(f, "Hal result: {err}"),
            Self::HalError(err) => write!(f, "Hal error: {err}"),
            Self::IoError(err) => write!(f, "I/O error: {err}"),
            Self::JsonError(err) => write!(f, "JSON error: {err}"),
            Self::SerialError(err) => write!(f, "Serial error: {err}"),
            Self::GenericError(err) => write!(f, "Generic error: {err}"),
            Self::Stop => write!(f, "Stop"),
        }
    }
}

/// Basic error type for JSON-RPC messages
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JsonRpcError {
    /// Error code (if present)
    pub(crate) code: JsonRpcErrorCode,
    /// Error message (if present)
    pub(crate) message: String,
}

impl JsonRpcError {
    /// Create a JsonRpcError
    pub fn new<C, S>(code: C, message: S) -> Self
    where
        C: Into<JsonRpcErrorCode>,
        S: Into<String>,
    {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    /// Create a JsonRpcError with a generic failure code
    pub fn failure<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(-1i64, message)
    }

    pub fn stop() -> Self {
        Self::new(JsonRpcErrorCode::Stop, "")
    }

    /// Get the error code
    pub fn code(&self) -> &JsonRpcErrorCode {
        &self.code
    }

    /// Set the error code
    pub fn set_code<C>(&mut self, code: C)
    where
        C: Into<JsonRpcErrorCode>,
    {
        self.code = code.into();
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Set the error message
    pub fn set_message<S>(&mut self, message: S)
    where
        S: Into<String>,
    {
        self.message = message.into();
    }
}

impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code[{}]: {}", self.code, self.message)
    }
}

impl From<HalResult> for JsonRpcError {
    fn from(res: HalResult) -> Self {
        match res {
            HalResult::Success => Self {
                code: res.into(),
                message: String::new(),
            },
            HalResult::InvalidHandle => Self {
                code: HalError::PreconditionFailed.into(),
                message: "Device handle is invalid".into(),
            },
            HalResult::ErrorInvalidArguments => Self {
                code: HalError::InternalValidationErr.into(),
                message: "One of the passed arguments is NULL".into(),
            },
            HalResult::IncompleteArguments => Self {
                code: HalError::InternalValidationErr.into(),
                message: "One of the passed arguments is incomplete (wrong length)".into(),
            },
            hal_result => Self {
                code: hal_result.into(),
                message: String::new(),
            },
        }
    }
}

#[cfg(feature = "std")]
impl From<JsonRpcError> for std::io::Error {
    fn from(err: JsonRpcError) -> Self {
        Self::new(std::io::ErrorKind::Other, format!("{err}"))
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for JsonRpcError {
    fn from(err: std::io::Error) -> Self {
        Self {
            code: err.into(),
            message: String::new(),
        }
    }
}

impl From<serde_json::Error> for JsonRpcError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            code: err.into(),
            message: String::new(),
        }
    }
}

impl From<std::str::Utf8Error> for JsonRpcError {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::failure(format!("{err}"))
    }
}

/// Basic error type for serial communication
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    code: ErrorCode,
    message: String,
}

impl Error {
    /// Create a generic failure Error
    pub fn failure<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            code: ErrorCode::Failure,
            message: message.into(),
        }
    }

    /// Create a serial port failure Error
    pub fn serial<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            code: ErrorCode::SerialPort,
            message: message.into(),
        }
    }

    /// Create a JSON-RPC failure Error
    pub fn json_rpc<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            code: ErrorCode::JsonRpc,
            message: message.into(),
        }
    }

    /// Get the error code
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, message: {}", self.code, self.message)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            code: ErrorCode::Failure,
            message: format!("I/O error: {}", err),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self {
            code: ErrorCode::Failure,
            message: format!("Utf8 error: {}", err),
        }
    }
}

impl From<serialport::Error> for Error {
    fn from(err: serialport::Error) -> Self {
        Self {
            code: ErrorCode::SerialPort,
            message: format!("Serial port error: {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        Self::failure(format!("failed to send an item to the queue: {err}"))
    }
}

impl From<&Error> for JsonRpcError {
    fn from(err: &Error) -> Self {
        Self::new(err.code(), err.message())
    }
}

impl From<Error> for JsonRpcError {
    fn from(err: Error) -> Self {
        Self::from(&err)
    }
}

impl From<JsonRpcError> for Error {
    fn from(err: JsonRpcError) -> Self {
        Self::from(&err)
    }
}

impl From<&JsonRpcError> for Error {
    fn from(err: &JsonRpcError) -> Self {
        Self::json_rpc(format!("code[{}]: {}", err.code(), err.message()))
    }
}

/// Error codes for returned errors from acceptor device
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
// FIXME: fill out with more error codes
pub enum ErrorCode {
    /// Generic failure code
    Failure = -1,
    /// Failure code originating from the serial port connection
    SerialPort = -2,
    /// JSON-RPC failure code
    JsonRpc = -3,
}

impl From<ErrorCode> for &'static str {
    fn from(e: ErrorCode) -> Self {
        match e {
            ErrorCode::Failure => "failure",
            ErrorCode::SerialPort => "serial port",
            ErrorCode::JsonRpc => "JSON-RPC",
        }
    }
}

impl From<&ErrorCode> for &'static str {
    fn from(e: &ErrorCode) -> Self {
        (*e).into()
    }
}

impl From<ErrorCode> for JsonRpcErrorCode {
    fn from(e: ErrorCode) -> Self {
        JsonRpcErrorCode::SerialError(format!("{e}"))
    }
}

impl From<&ErrorCode> for JsonRpcErrorCode {
    fn from(e: &ErrorCode) -> Self {
        (*e).into()
    }
}

impl From<JsonRpcErrorCode> for ErrorCode {
    fn from(_e: JsonRpcErrorCode) -> Self {
        Self::JsonRpc
    }
}

impl From<&JsonRpcErrorCode> for ErrorCode {
    fn from(_e: &JsonRpcErrorCode) -> Self {
        Self::JsonRpc
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
