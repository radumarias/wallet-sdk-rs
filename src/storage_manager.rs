/// Type: Key
///
/// Type alias for Keys, it's just a String
type Key = String;


/// Type: Value
///
/// Type alias for Values, it's just a String
type Value = String;

/// Class: StorageManager
///
/// This is the Rust side of the storage manager.  The real work is performed in
/// the kotlin and swift functions, but this exposes that in a platform neutral
/// way to our rust code.
///
/// The native SecureStorage system is just has a Strings for Keys, and Strings for Values.
/// It's probably best to think of it as local-storage in your browser.
pub struct StorageManager {
    /// Var: native_add_fn
    ///
    /// Callback function pointer to native (kotlin/swift) code for adding
    /// a key.
    native_add_fn: FnMut(Key, Value) -> Result<Error, Unit>,

    /// Var: native_get_fn
    ///
    /// Callback function pointer to native (kotlin/swift) code for
    /// getting a key.
    native_get_fn: FnMut(Key) -> Result<Error, Value>,

    /// Var: native_remove_fn
    ///
    /// Callback function pointer to native (kotlin/swift) code for
    /// removing a key.  This referenced function MUST be idempotent.  In
    /// particular, it must treat removing a non-existent key as a normal and
    /// expected circumstance, simply returning Unit and not an error.
    native_remove_fn: FnMut(Key) -> Result<Error, Unit>,
}

impl StorageManager {
    /// Method: add
    ///
    /// Adds Value for Key.  Should the key already exist, it is replaced
    ///
    /// Arguments:
    /// key - <Key> to store the value under
    /// value - <Value> to store
    fn add(key: Key, value: Value) -> Result<Error, Unit>;

    /// Method: get
    ///
    /// Attempts to fetch a value stored under Key from SecureStorage,
    /// Should the value not exist, None is returned
    ///
    /// Arguments:
    /// key - <Key> we look under in SecureStorage
    ///
    /// Return:
    /// None - when the key is not present in SecureStorage
    /// Some(<Value>) - when the key is present
    fn get(key: Key) -> Result<Error, Option<Value>>;


    /// Method: remove
    ///
    /// Removes the <Key> - <Value> pair from Secure Storage.  Should the key
    /// not be present in SecureStorage, no error is raised.
    ///
    /// Arguments:
    /// key - <Key> to remove
    ///
    /// Return:
    /// () in all cases
    fn remove(key: Key) -> Result<Error, Unit>;


}
