mod holder;
mod nfc_bluetooth_holder;
mod qr_bluetooth_holder;

pub use crate::presentment::holder::holder::Holder;
use crate::presentment::holder::nfc_bluetooth_holder::new_nfc_bluetooth_holder;
use crate::presentment::holder::qr_bluetooth_holder::new_qr_bluetooth_holder;
use crate::presentment::{EngagementMethod, PresentmentMethod, TransmissionMethod};

/// Struct: TransmissionResult
///
/// This is the result of attempting to share an mDL.  After receiving a TransmissionResult,
/// the UX will need to present something to the user.
pub struct TransmissionResult(());

/// Function: newTransmission
///
/// Factory function that interprets the contents of transmissionMethod to return an appropriate
/// Holder.
///
/// Arguments:
/// presentment_method - PresentmentMethod describing how the MDL will be presented
///
/// Returns:
/// Always returns a Holder.
pub fn new_holder(presentment_method: PresentmentMethod) -> Box<dyn Holder> {
    match (
        presentment_method.engagement_method,
        presentment_method.transmission_method,
    ) {
        (EngagementMethod::QrCode, TransmissionMethod::BLE(_)) => {
            Box::new(new_qr_bluetooth_holder())
        }
        (EngagementMethod::NFC(_), TransmissionMethod::BLE(_)) => {
            Box::new(new_nfc_bluetooth_holder())
        }
        (_, _) => panic!("Not Yet Implemented"),
    }
}
