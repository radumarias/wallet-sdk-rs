/// Class: NFC
/// 
/// Implements the APDU command parsing.
impl NFC {

  /// Function: process_command_apdu
  /// 
  /// Top entry point that will process the apdu command and pass it to a handler 
  /// for processing.
  /// 
  /// This is connected with services `processCommandApdu` event.
  pub fn process_command_apdu(mut self, apdu: Vec<u8>) -> Vec<u8> {

  }

  /// Function: get_deactivation_reason
  /// 
  /// Maps termination reason from apdu command.
  /// 
  /// This is connected with services `onDeactivated` event.
  pub fn get_deactivation_reason(reason: i32) -> DeactivationReason {
  
  }

  /// Function: get_command_type
  /// 
  /// Processes the apdu commands by looking at the instruction and parameter P1. 
  /// 
  /// As described in CSA ISO/IEC 7816-4 - 5.2 Table 1.
  fn get_command_type(apdu: Vec<u8>) -> i32 {

  }

  /// Function: handle_select_by_aid
  /// 
  /// HCE service selecting NFC Data Exchange Format.
  /// 
  /// As defined in NFC Forum Type 4 Tag Technical Specification v1.2 
  fn handle_select_by_aid(apdu: Vec<u8>) -> Vec<u8> {

  }

  /// Function: handle_select_file
  /// 
  /// Selecting supported file. Currently, we support Tag 4 with static NDEF.
  /// 
  fn handle_select_file(&mut self, apdu: Vec<u8>) -> Vec<u8> {

  }

  /// Function: handle_read_binary
  /// 
  /// Transmits file data. The selectedNFCFile is dynamically constructed in file selection.
  /// 
  /// As defined in the Connection Handover Technical Specification v1.5 
  fn handle_read_binary(self, apdu: Vec<u8>) -> Vec<u8> {

  }
}