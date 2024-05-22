/// Function: newQrBluetoothReader
///
/// Creates a new instance of a QrBluetoothReader
pub fn newQrBluetoothReader() -> QrBluetoothReader {
}


/// Class: QrBluetoothReader
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
pub struct QrBluetoothReader(());
impl QrBluetoothReader {
    // /// Function: initialize_session
    // ///
    // /// Selects the requested presentment data.
    // pub fn initialize_session() {

    //     // Generates the session with the request
    // }

    // // This gets absorbed by the initialize.
    // pub fn start_engagement() {
    //     // This would use a ble or nfc manager to start the engagement
    //     //
    //     // Advertisement
    //     // Generate Request
    // }

    // pub fn handle_response() {
    //     // Validates the response data with the certificate.
    // }

    // /// Function: open_qr_code_scanner
    // ///
    // ///
    // pub fn open_qr_code_scanner() {

    // }

    // /// Function: start_advertisement
    // ///
    // /// Starts BLE service advertisement on the provided UUID
    // pub fn start_advertisement() {

    // }

    // /// Function: generate_request
    // ///
    // /// Validation of the BLE connection
    // pub fn generate_request() {

    // }

    // /// Function: handle_response
    // ///
    // /// Processes the response and validates against the certificate. This is whatever the user trims
    // /// down.
    // pub fn handle_response() {}

    // /// Function: terminate_session
    // ///
    // /// Disconnects the BLE service
    // pub fn terminate_session() {}

    // /// Function: open_qr_scan
    // ///
    // /// Opens the QR code scanner to extract information and start the BLE transaction.
    // fn open_qr_scan() {}
}

impl Reader for BluetoothReader {
    fn retrieve(self) {
        self.open_qr_scanner();
        self.transport_handover();
    }

    fn validate() {}
}
