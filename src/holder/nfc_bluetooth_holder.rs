/// Function: newNfcBluetoothHolder
///
/// Creates a new instance of a NfcBluetoothHolder
pub fn newNfcBluetoothReader() -> NfcBluetoothHolder {
}


/// Class: NfcBluetoothHolder
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
pub struct NfcBluetoothHolder(());

impl NfBluetoothHolder {
    
}

impl Reader for BluetoothHolder {
    fn retrieve(self) {
        self.open_qr_scanner();
        self.transport_handover();
    }

    fn confirm() {}
}


// /// Class: Bluetooth
// /// 
// /// Implements the bluetooth for presentment.
// impl BluetoothHolder {

//   /// Function: initialize_session
//   /// 
//   /// Generates session, QR code and ble_ident with document and UUID
//   fn initialize_session(document: Arc<MDoc>, uuid: Uuid) -> Result<SessionData, SessionError> {

//   }

//   /// Function: handle_request
//   /// 
//   fn handle_request(
//     state: Arc<SessionManagerEngaged>,
//     request: Vec<u8>) -> Result<RequestData, RequestError> {

//   }

//   /// Function: submit_response
//   /// 
//   fn submit_response(
//     session_manager: Arc<SessionManager>,
//     permitted_items: HashMap<String, HashMap<String, Vec<String>>>
//   ) -> Result<Vec<u8>, ResponseError> {

//   }

//   /// Function: submit_signature
//   /// 
//   /// 
//   fn submit_signature(
//     session_manager: Arc<SessionManager>,
//     der_signature: Vec<u8>) -> Result<Vec<u8>, SignatureError> {

//   }

//   /// Function: terminate_session
//   /// 
//   /// Terminates the BLE connection.
//   fn terminate_session() -> Result<Vec<u8>, TerminationError> {

//   }
// }