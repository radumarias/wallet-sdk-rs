/// Function: newQrBluetoothHolder
///
/// Creates a new instance of a QrBluetoothHolder
pub fn newQrBluetoothReader() -> QrBluetoothHolder {
}


/// Class: QrBluetoothHolder
///
/// This is the BLE specific implementation
///
/// Get BLE holder trait as it would return from a presentment connection manager.
///
/// Create a generic holder trait that would return the instance of the BluetoothReader trait
/// as one of the possible options.
///
/// Reader specific presentment methods - presentment over BLE.
///
/// Implements the <Holder> trait.
pub struct QrBluetoothHolder(());

impl QrBluetoothHolder {
    
}

impl Reader for BluetoothHolder {
    fn retrieve(self) {
        self.open_qr_scanner();
        self.transport_handover();
    }

    fn confirm() {}
}