pub struct TransmissionResult(());

/// Function: newTransmission
///
/// Factory function that interprets the contents of transmissionMethod to return an appropriate
/// Holder.
///
/// Arguments:
/// transmissionMethod - TransmissionMethod describing how we wish to transmit the credentials
///
/// Returns:
/// Always returns a Holder.
pub fn newHolder(transmissionMethod: TransmissionMethod) -> Holder {
    match (
        transmissionMethod.engagementMethod,
        transmissionMethod.transmissionMethod,
    ) {
        (qrCode, ble) => newQrBluetoothHolder(),
        (nfc(_), ble) => newNfcBluetoothHolder(),
        (_, _) => panic!("Not Yet Implemented"),
    }
}

/// Interface: Holder
///
/// Interface implemented by different holder types.  While it is possible to use a specific
/// holder directly, it's best to use the <newHolder> function to create one based on the
/// transmissionMethod you desire.
pub trait Holder {
  /// Method: transmit
  ///
  ///
  pub fn transmit(progressUpdateCallback: ProgressUpdateFunction) -> RetrievalResult;

  /// Method: confirm
  ///
  pub fn confirm(retrievalResult: RetrievalResult, progressUpdateCallback: ProgressUpdateFunction);
}





// /// Holder specific presentment methods.
// impl Holder {
//   /// NFC APDU command processor that connects with the NFC service.
//   fn process_command_apdu() {

//   }

//   /// BLE 
//   fn initialize_session() {

//   }

//   /// BLE
//   fn handle_request() {

//   }

//   // BLE
//   fn submit_response() {

//   }

//   /// BLE
//   fn terminate_session() {

//   }
// }
