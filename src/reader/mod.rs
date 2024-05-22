pub struct RetrievalResult(());

/// Function: newReader
///
/// Factory function that interprets the contents of retrievalMethod to return an appropriate
/// Reader.
///
/// Arguments:
/// retrievalMethod - RetrievalMethod describing how we wish to retrieve the credentials
///
/// Returns:
/// Always returns a Reader.
pub fn newReader(retrievalMethod: RetrievalMethod) -> Reader {
    match (
        retrievalMethod.engagementMethod,
        retrievalMethod.transmissionMethod,
    ) {
        (qrCode, ble) => newQrBluetoothReader(),
        (nfc(_), ble) => newNfcBluetoothReader(),
        (_, _) => panic!("Not Yet Implemented"),
    }
}

/// Interface: Reader
///
/// Interface implemented by different reader types.  While it is possible to use a specific
/// reader directly, it's best to use the <newReader> function to create one based on the
/// retrievalMethod you desire.
pub trait Reader {
    /// Method: retrieve
    ///
    /// Optional method which Pops open QRCode or NFC Reader to scan the date
    /// The user can use their own QR Code scanner or NFC Reader to build a scan result with though
    pub fn retrieve(progressUpdateCallback: ProgressUpdateFunction) -> RetrievalResult;

    /// Method: validate
    ///
    /// Uses the retrieval result to establish the transport connection, receives the mDL and
    /// validates it.
    pub fn validate(
        retrievalResult: RetrievalResult,
        progressUpdateCallback: ProgressUpdateFunction,
    );
}
