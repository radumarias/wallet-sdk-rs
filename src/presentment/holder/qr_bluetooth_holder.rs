use crate::presentment::holder::Holder;
use crate::presentment::holder::TransmissionResult;
use crate::presentment::PresentmentError;
use crate::ProgressUpdateFunction;

/// Function: newQrBluetoothHolder
///
/// Creates a new instance of a QrBluetoothHolder
pub fn new_qr_bluetooth_holder() -> QrBluetoothHolder {
    QrBluetoothHolder(())
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

impl QrBluetoothHolder {}

impl Holder for QrBluetoothHolder {
    fn transmit(
        &self,
        progress_update_callback: &ProgressUpdateFunction,
    ) -> Result<TransmissionResult, PresentmentError> {
        panic!("Not implemented");
    }

    fn confirm(
        &self,
        transmission_result: TransmissionResult,
        progress_update_callback: &ProgressUpdateFunction,
    ) -> Result<(), PresentmentError> {
        panic!("Not implemented");
    }
}
