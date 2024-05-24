// This is here since we're just stubbing out code right now, and NONE of the params will be used
// It should be removed after we're done with implementation though
#![allow(unused)]

pub mod presentment;
pub mod storage_manager;

uniffi::setup_scaffolding!();

/// Type: ProgressUpdateFunction
///
/// This is a callback function used to push progress updates up to the client code (UI) in near
/// real time.
type ProgressUpdateFunction = dyn FnMut();

/// Function: blah_version
///
/// Returns the current version of this lib
///
/// Returns:
/// String representation of the version of this lib
#[uniffi::export]
#[allow(nonstandard_style)]
pub fn blah_version() -> String {
    "1.0.0".to_string()
}

/// Function: camelVersion
///
/// Returns the current version of this lib
///
/// Returns:
/// String representation of the version of this lib
#[uniffi::export]
#[allow(nonstandard_style)]
pub fn camelVersion() -> String {
    "1.0.0".to_string()
}
