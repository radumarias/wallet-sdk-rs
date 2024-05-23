use crate::presentment::reader::RetrievalResult;
use crate::presentment::reader::ValidationResult;
use crate::presentment::PresentmentError;
use crate::ProgressUpdateFunction;

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
    fn retrieve(&self, progress_update_callback: &ProgressUpdateFunction) -> RetrievalResult;

    /// Method: validate
    ///
    /// Uses the retrieval result to establish the transport connection, receives the mDL and
    /// validates it against any certificate in the CertificateManager
    fn validate(
        &self,
        retrieval_result: RetrievalResult,
        progress_update_callback: &ProgressUpdateFunction,
    ) -> Result<ValidationResult, PresentmentError>;

    /// Method: validate_with_cert
    ///
    /// Uses the retrieval result to establish the transport connection, receives the mDL and
    /// validates it against the specified certificate.
    fn validate_with_cert(
        &self,
        retrieval_result: RetrievalResult,
        progress_update_callback: &ProgressUpdateFunction,
        certificate_fingerprint_id: (), // Should ultimately be the CertificateFingerprintId
    ) -> Result<ValidationResult, PresentmentError>;
}
