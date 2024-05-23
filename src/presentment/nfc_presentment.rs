/// Enum: NFCEngagementOptions
///
/// Configuration option for NFC engagement method.
///
/// StaticHandover - The Presenter behaves as a normal NFC tag, with no dynamic interchange.  This
///                  mode is less secure than negotiated_handover
/// NegotiatedHandover - The presenter communicates in full duplex with the receiver.  This is
///                      more secure.
pub enum NFCEngagementOptions {
    StaticHandover,
    NegotiatedHandover,
}

/// Struct: NFCTransmissionOptions
///
/// Struct used to communicate NFC transmission options to underlying functions.
pub struct NFCTransmissionOptions(());
