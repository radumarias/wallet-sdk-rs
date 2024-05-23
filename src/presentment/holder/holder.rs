use crate::presentment::holder::TransmissionResult;
use crate::presentment::PresentmentError;
use crate::ProgressUpdateFunction;

/// Interface: Holder
///
/// Interface implemented by different holder types.  While it is possible to use a specific
/// holder directly, it's best to use the <newHolder> function to create one based on the
/// transmissionMethod you desire.
pub trait Holder {
    /// Method: transmit
    ///
    /// This should be called from the holder side to begin attempting to present an mDL
    /// to a reader.  The Transmission result MAY contain a QR code which should be displayed
    /// to the holder if QR engagement is selected.
    fn transmit(
        &self,
        progress_update_callback: &ProgressUpdateFunction,
    ) -> Result<TransmissionResult, PresentmentError>;

    /// Method: confirm
    ///
    /// This confirms which data elements to actually send to the reader.  For instance,
    /// Name, Birthdate, Age, etc...  The User should be presented with a screen to get
    /// confirmation on which data elements to send.
    ///
    /// The Reader will initially request a set of data elements
    fn confirm(
        &self,
        transmission_result: TransmissionResult,
        progress_update_callback: &ProgressUpdateFunction,
    ) -> Result<(), PresentmentError>;
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
