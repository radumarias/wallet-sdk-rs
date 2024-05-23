use crate::presentment::reader::Reader;
use crate::presentment::reader::{RetrievalResult, ValidationResult};
use crate::presentment::PresentmentError;
use crate::ProgressUpdateFunction;

/// Function: newNfcBluetoothReader
///
/// Creates a new instance of a NfcBluetoothReader
pub fn new_nfc_bluetooth_reader() -> NFCBluetoothReader {
    panic!("Not Implemented")
}

/// Class: NfcBluetoothReader
///
/// This is the BLE specific implementation
///
/// Get BLE reader trait as it would return from a presentment connection manager.
///
/// Create a generic reader trait that would return the instance of the BluetoothReader trait
/// as one of the possible options.
///
/// Reader specific presentment methods - presentment over BLE.
///
/// Implements the <Reader> trait.
pub struct NFCBluetoothReader(());

impl NFCBluetoothReader {}

impl Reader for NFCBluetoothReader {
    fn retrieve(&self, progress_update_callback: &ProgressUpdateFunction) -> RetrievalResult {
        panic!("Not Implemented")
    }

    fn validate(
        &self,
        retrieval_result: RetrievalResult,
        progress_update_callback: &ProgressUpdateFunction,
    ) -> Result<ValidationResult, PresentmentError> {
        panic!("Not Implemented")
    }

    fn validate_with_cert(
        &self,
        retrieval_result: RetrievalResult,
        progress_update_callback: &ProgressUpdateFunction,
        certificate_fingerprint_id: (), // Should ultimately be the CertificateFingerprintId
    ) -> Result<ValidationResult, PresentmentError> {
        panic!("Not Implemented")
    }
}
