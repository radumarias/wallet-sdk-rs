use thiserror::Error;

use crate::platform_context::PlatformContext;
use crate::storage_manager::StorageManagerError;

// Struct: CertificateFingerprintId
//
// Fingerprint of the certificate
uniffi::custom_newtype!(CertificateFingerprintId, Vec<u8>);
pub struct CertificateFingerprintId(Vec<u8>);

// We don't use the types in x509-cert because we need these to cross FFI boundaries
// into Kotlin and Swift.
pub enum Certificate {
    Pem(String),
    Der(Vec<u8>),
}

// Represents errors that may occur during certificate management operations
#[derive(Error, Debug, uniffi::Error)]
pub enum CertificateManagerError {
    // An error occurred in the underlying storage manager
    #[error("Storage Manager Error")]
    StorageManager(StorageManagerError),
}

// We just introduce the certificate manager as an object because it makes things easier
// in uniffi.  We're essentially in a global namespace, and would potentially have collisions
// with add.
#[derive(uniffi::Object)]
pub struct CertificateManager(());

//#[uniffi::export]
impl CertificateManager {
    // Function: add
    //
    // Persists the certificate as is becomes available via issuance or some other process.  Provided
    // certificate material, persists the certificate and returns an alias.
    //
    // Arguments:
    // certificate - <Certificate> to add
    // storageManager - <StorageManager> used to interact with native storage
    //
    // Return:
    // Result of Error - when an error occurred, this is most likely to occur in the underlying storage
    //                   mechanism
    // Result of <CertificateFingerprintId> - when everything functions normally.
    pub fn add(
        ctx: &PlatformContext,
        certificate: Certificate,
    ) -> Result<CertificateFingerprintId, CertificateManagerError> {
        panic!("Not Implemented")
    }

    // Function: get
    //
    // Retrieves the certificate by <CertificateFingerprintId> to be used with a verification strategy.
    //
    // Arguments:
    // id - <CertificateFingerprintId>
    // storageManager - <StorageManager> used to lookup the certificate
    //
    // Return:
    // Result of Error - When an error is encountered, this is most likely to occur in the underlying
    //                   native code.  This IS NOT returned simply because the certificate could not
    //                   be found.
    // Result of Some(<Certificate>) - When no errors are encountered, and we were able to find
    //                                 a certificate.
    // Result of None - When no errors are encountered, but we could not find the certificate id.
    pub fn get(
        ctx: &PlatformContext,
        id: CertificateFingerprintId,
    ) -> Result<Option<Certificate>, CertificateManagerError> {
        Ok(Some(Certificate::Pem(
            "Certificate Fingerprint Id".to_string(),
        )))
    }

    // Function: revoke
    //
    // Marks the credential as revoked - does not remove the actual credential to preserve the record.
    pub fn revoke(
        ctx: &PlatformContext,
        id: CertificateFingerprintId,
    ) -> Result<(), CertificateManagerError> {
        Ok(())
    }

    // Function: remove
    //
    // Removes the certificate by alias.
    pub fn remove(
        ctx: &PlatformContext,
        id: CertificateFingerprintId,
    ) -> Result<(), CertificateManagerError> {
        Ok(())
    }

    // Function: is_certificate_signature_valid
    //
    // Verifies that the certificate is valid and not revoked.
    // Note: intended to be used to verify the validity of the certificate.
    pub fn is_certificate_valid(
        ctx: &PlatformContext,
        id: CertificateFingerprintId,
    ) -> Result<bool, CertificateManagerError> {
        Ok(false)
    }

    // Function: is_certificate_signature_valid
    //
    // Validates the signature of data against the certificate.
    // Note: intended to be used with readers/presentment.
    pub fn is_certificate_signature_valid(
        ctx: &PlatformContext,
        id: CertificateFingerprintId,
        data: Vec<u8>,
    ) -> Result<bool, CertificateManagerError> {
        Ok(false)
    }
}
