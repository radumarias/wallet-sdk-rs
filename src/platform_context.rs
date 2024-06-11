use crate::storage_manager::StorageManagerInterface;
use std::sync::Arc;

/// Many functions on Android will require a pointer to the android context, so we declare a type
/// in rust so we can track it with a type.
///
/// Here we use a callback interface to model the reference to an android context.  We don't need
/// any functions, since rust code should never meddle with this.
#[uniffi::export(callback_interface)]
pub trait AndroidContext: Send + Sync {}

/// The PlatformContext provides functions we use to access functionality from the underlying
/// native platform.
///
/// We use the older callback_interface to keep the required version level of our Android API
/// low.
///
/// Note that the interfaces use the older 'callback_interface' export so that we can use
/// older versions of the Android SDK.  This type of interace requires the use of Box, not Arc
#[derive(uniffi::Object)]
#[allow(dead_code)]
pub struct PlatformContext {
    pub android_context: Option<Box<dyn AndroidContext>>,
    pub storage_manager: Box<dyn StorageManagerInterface>,
}

#[uniffi::export]
impl PlatformContext {
    #[uniffi::constructor]
    fn new(
        android_context: Option<Box<dyn AndroidContext>>,
        storage_manager_interface: Box<dyn StorageManagerInterface>,
    ) -> Arc<Self> {
        PlatformContext {
            android_context: android_context,
            storage_manager: storage_manager_interface,
        }
        .into()
    }
}
