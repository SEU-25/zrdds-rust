//! DDS Return Code definitions for Rust
//!
//! This module provides a Rust-idiomatic enum representation of DDS_ReturnCode_t
//! along with conversion utilities and error handling.

use crate::bindings::DDS_ReturnCode_t;
use std::fmt;

/// Rust representation of DDS_ReturnCode_t
///
/// This enum provides a type-safe, Rust-idiomatic way to handle DDS return codes.
/// It can be converted to/from the raw C DDS_ReturnCode_t values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ReturnCode {
    /// Operation completed successfully
    Ok = 0,
    /// Generic error occurred
    Error = 1,
    /// Operation is not supported
    Unsupported = 2,
    /// Invalid parameter provided
    BadParameter = 3,
    /// Precondition not met for operation
    PreconditionNotMet = 4,
    /// Insufficient resources available
    OutOfResources = 5,
    /// Entity is not enabled
    NotEnabled = 6,
    /// Attempt to modify immutable policy
    ImmutablePolicy = 7,
    /// Inconsistent QoS policies
    Inconsistent = 8,
    /// Attempt to delete already deleted entity
    AlreadyDeleted = 9,
    /// Operation timed out
    Timeout = 10,
    /// No data available
    NoData = 11,
    /// Illegal operation attempted
    IllegalOperation = 12,
    /// Operation not allowed by security
    NotAllowedBySec = 13,
}

impl ReturnCode {
    /// Check if the return code indicates success
    pub fn is_ok(&self) -> bool {
        matches!(self, ReturnCode::Ok)
    }

    /// Check if the return code indicates an error
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Get a human-readable description of the return code
    pub fn description(&self) -> &'static str {
        match self {
            ReturnCode::Ok => "Operation completed successfully",
            ReturnCode::Error => "Generic error occurred",
            ReturnCode::Unsupported => "Operation is not supported",
            ReturnCode::BadParameter => "Invalid parameter provided",
            ReturnCode::PreconditionNotMet => "Precondition not met for operation",
            ReturnCode::OutOfResources => "Insufficient resources available",
            ReturnCode::NotEnabled => "Entity is not enabled",
            ReturnCode::ImmutablePolicy => "Attempt to modify immutable policy",
            ReturnCode::Inconsistent => "Inconsistent QoS policies",
            ReturnCode::AlreadyDeleted => "Attempt to delete already deleted entity",
            ReturnCode::Timeout => "Operation timed out",
            ReturnCode::NoData => "No data available",
            ReturnCode::IllegalOperation => "Illegal operation attempted",
            ReturnCode::NotAllowedBySec => "Operation not allowed by security",
        }
    }
}

impl From<DDS_ReturnCode_t> for ReturnCode {
    fn from(code: DDS_ReturnCode_t) -> Self {
        match code {
            0 => ReturnCode::Ok,
            1 => ReturnCode::Error,
            2 => ReturnCode::Unsupported,
            3 => ReturnCode::BadParameter,
            4 => ReturnCode::PreconditionNotMet,
            5 => ReturnCode::OutOfResources,
            6 => ReturnCode::NotEnabled,
            7 => ReturnCode::ImmutablePolicy,
            8 => ReturnCode::Inconsistent,
            9 => ReturnCode::AlreadyDeleted,
            10 => ReturnCode::Timeout,
            11 => ReturnCode::NoData,
            12 => ReturnCode::IllegalOperation,
            13 => ReturnCode::NotAllowedBySec,
            _ => ReturnCode::Error, // Default to generic error for unknown codes
        }
    }
}

impl From<ReturnCode> for DDS_ReturnCode_t {
    fn from(code: ReturnCode) -> Self {
        code as DDS_ReturnCode_t
    }
}

impl fmt::Display for ReturnCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self, self.description())
    }
}

impl std::error::Error for ReturnCode {}

/// Result type alias for DDS operations
///
/// This provides a convenient way to return either a success value or a DDS error code.
pub type DdsResult<T> = Result<T, ReturnCode>;

/// Utility function to convert a raw DDS_ReturnCode_t to a Rust Result
///
/// # Arguments
/// * `code` - The raw DDS return code
/// * `value` - The value to return on success
///
/// # Returns
/// * `Ok(value)` if code indicates success
/// * `Err(ReturnCode)` if code indicates an error
pub fn dds_result<T>(code: DDS_ReturnCode_t, value: T) -> DdsResult<T> {
    let return_code = ReturnCode::from(code);
    if return_code.is_ok() {
        Ok(value)
    } else {
        Err(return_code)
    }
}

/// Utility function to convert a raw DDS_ReturnCode_t to a Rust Result for unit operations
///
/// # Arguments
/// * `code` - The raw DDS return code
///
/// # Returns
/// * `Ok(())` if code indicates success
/// * `Err(ReturnCode)` if code indicates an error
pub fn dds_result_unit(code: DDS_ReturnCode_t) -> DdsResult<()> {
    dds_result(code, ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_code_conversion() {
        // Test conversion from DDS_ReturnCode_t to ReturnCode
        assert_eq!(ReturnCode::from(0), ReturnCode::Ok);
        assert_eq!(ReturnCode::from(1), ReturnCode::Error);
        assert_eq!(ReturnCode::from(13), ReturnCode::NotAllowedBySec);

        // Test conversion from ReturnCode to DDS_ReturnCode_t
        assert_eq!(DDS_ReturnCode_t::from(ReturnCode::Ok), 0);
        assert_eq!(DDS_ReturnCode_t::from(ReturnCode::Error), 1);
        assert_eq!(DDS_ReturnCode_t::from(ReturnCode::NotAllowedBySec), 13);
    }

    #[test]
    fn test_is_ok_is_err() {
        assert!(ReturnCode::Ok.is_ok());
        assert!(!ReturnCode::Ok.is_err());

        assert!(!ReturnCode::Error.is_ok());
        assert!(ReturnCode::Error.is_err());
    }

    #[test]
    fn test_dds_result() {
        let result = dds_result(0, "success");
        assert_eq!(result, Ok("success"));

        let result = dds_result(1, "error");
        assert_eq!(result, Err(ReturnCode::Error));
    }

    #[test]
    fn test_dds_result_unit() {
        let result = dds_result_unit(0);
        assert_eq!(result, Ok(()));

        let result = dds_result_unit(1);
        assert_eq!(result, Err(ReturnCode::Error));
    }

    #[test]
    fn test_display() {
        let code = ReturnCode::BadParameter;
        let display_str = format!("{}", code);
        assert!(display_str.contains("BadParameter"));
        assert!(display_str.contains("Invalid parameter provided"));
    }
}
