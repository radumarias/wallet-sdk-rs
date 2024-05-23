/// Struct: RetrievalMethod
///
/// Retrieval encapsulation including engagement, transmission and additional configuration.
pub struct RetrievalMethod {
    engagementMethod: EngagementMethod,
    transmissionMethod: TransmissionMethod,
}

/// Enum: EngagementMethod
///
/// For selecting the initial engagement method with another device
/// as part of 18013-5
///
/// nfc - Uses NFC to engage devices for presentment, requires NfcEngagementOptions
/// qrCode - Uses a QR code to engage devices for presentment
pub enum EngagementMethod {
    nfc(NfcEngagementOptions),
    qrCode,
}

/// Enum: TransmissionMethod
///
/// For selecting the data transmission method to another device
/// as part of 18013-5
///
/// nfc - Uses nfc to transmit credentials for presentment
/// ble - Uses bluetooth to transmit credentials for presentment
/// wifi_aware - Uses wifi to transmit credentials for presentment
pub enum TransmissionMethod {
    nfc(NfcTransmissionOptions),
    ble(BleTransmissionOptions),
    wifi_aware(WifiAwareTransmissionOptions),
}

/// Enum: BleTransmissionOptions
///
/// Configuration option for BLE transmission method.
///
/// central - Uses BLE Central mode for Transmission
/// peripheral - Uses BLE Peripheral mode for transmission
pub enum BleTransmissionOptions {
    central,
    peripheral,
}

/// Enum: NfcEngagementOptions
///
/// Configuration option for NFC engagement method.
///
/// static_handover - The Presenter behaves as a normal NFC tag, with no dynamic interchange.  This
///                    mode is less secure than negotiated_handover
/// negotiated_handover - The presenter communicates in full duplex with the receiver.  This is
///                       more secure.
pub enum NFCEngagementOptions {
    static_handover,
    negotiated_handover,
}
