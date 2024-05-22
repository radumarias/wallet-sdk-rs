/// Namespace: logger
///
/// This module contains functions for logging to the native console in kotlin or swift.
mod logger {
    /// Macro: trace!
    ///
    /// --- Prototype ---
    /// trace!(message: String)
    /// -----------------
    ///
    /// Logs trace out into the native console.
    ///
    /// Arguments:
    /// message - String to log
    macro_rules! trace {
        (message: String) => {};
    }

    /// Macro: info!
    ///
    /// --- Prototype ---
    /// info!(message: String)
    /// -----------------
    ///
    /// Logs info out into the native console.
    ///
    /// Arguments:
    /// message - String to log
    macro_rules! info {
        (message: String) => {};
    }

    /// Macro: warn!
    ///
    /// --- Prototype ---
    /// warn!(message: String)
    /// -----------------
    ///
    /// Logs warning out into the native console.
    ///
    /// Arguments:
    /// message - String to log
    macro_rules! warn {
        (message: String) => {};
    }

    /// Macro: error!
    ///
    /// --- Prototype ---
    /// error!(message: String)
    /// -----------------
    ///
    /// Logs error out into the native console.
    ///
    /// Arguments:
    /// message - String to log
    macro_rules! error {
        (message: String) => {};
    }
}

// Ex use:
// logger.warn("Message");
// logger.trace("Message");
