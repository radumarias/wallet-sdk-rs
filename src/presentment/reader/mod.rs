mod nfc_bluetooth_reader;
mod qr_bluetooth_reader;
mod reader;

use crate::presentment::reader::nfc_bluetooth_reader::new_nfc_bluetooth_reader;
use crate::presentment::reader::qr_bluetooth_reader::new_qr_bluetooth_reader;
pub use crate::presentment::reader::reader::Reader;
use crate::presentment::{EngagementMethod, PresentmentMethod, TransmissionMethod};

/// Function: newReader
///
/// Factory function that interprets the contents of retrievalMethod to return an appropriate
/// Reader.
///
/// Arguments:
/// presentment_method - <PresentmentMethod> describing how we wish to retrieve the credentials
///
/// Returns:
/// Always returns a Reader.
pub fn new_reader(presentment_method: PresentmentMethod) -> Box<dyn Reader> {
    match (
        presentment_method.engagement_method,
        presentment_method.transmission_method,
    ) {
        (EngagementMethod::QrCode, TransmissionMethod::BLE(_)) => {
            Box::new(new_qr_bluetooth_reader())
        }
        (EngagementMethod::NFC(_), TransmissionMethod::BLE(_)) => {
            Box::new(new_nfc_bluetooth_reader())
        }
        (_, _) => panic!("Not Yet Implemented"),
    }
}

/// Struct: RetrievalResult
pub struct RetrievalResult(());

/// Struct: ValidationResult
pub struct ValidationResult(());
