mod bluetooth_presentment;
mod holder;
mod nfc_presentment;
mod reader;
mod wifi_aware_presentment;
pub use crate::presentment::bluetooth_presentment::BLETransmissionOptions;
pub use crate::presentment::nfc_presentment::{NFCEngagementOptions, NFCTransmissionOptions};
pub use crate::presentment::wifi_aware_presentment::WifiAwareTransmissionOptions;
use thiserror::Error;

/// Enum: EngagementMethod
///
/// For selecting the initial engagement method with another device
/// as part of 18013-5
///
/// NFC - Uses NFC to engage devices for presentment, requires NfcEngagementOptions
/// QrCode - Uses a QR code to engage devices for presentment
pub enum EngagementMethod {
    NFC(NFCEngagementOptions),
    QrCode,
}

/// Enum: TransmissionMethod
///
/// For selecting the data transmission method to another device
/// as part of 18013-5
///
/// NFC - Uses nfc to transmit credentials for presentment
/// BLE - Uses bluetooth to transmit credentials for presentment
/// WifiAware - Uses wifi to transmit credentials for presentment
pub enum TransmissionMethod {
    NFC(NFCTransmissionOptions),
    BLE(BLETransmissionOptions),
    WifiAware(WifiAwareTransmissionOptions),
}

/// Struct: PresentmentMethod
///
/// PresentmentMethod encapsulation including engagement, transmission and additional configuration.
pub struct PresentmentMethod {
    /// Variable: engagementMethod
    /// The <EngagementMethod> to use during presentment
    pub engagement_method: EngagementMethod,

    /// Variable: transmissionMethod
    /// The <TransmissionMethod> to use during presentment
    pub transmission_method: TransmissionMethod,
}

/// Enum: PresentmentError
///
/// Represents errors that may occur during presentment
#[derive(Error, Debug)]
pub enum PresentmentError {
    #[error("Stub error")]
    Stub,
}
