/// Function: newNfcBluetoothReader
///
/// Creates a new instance of a NfcBluetoothReader
pub fn newNfcBluetoothReader() -> NfcBluetoothReader {
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
pub struct NfcBluetoothReader(());

impl NfcBluetoothReader {
    
}

impl Reader for BluetoothReader {
    fn retrieve(self) {
        self.open_qr_scanner();
        self.transport_handover();
    }

    fn validate() {}
}
