//! Storage for the Pinboard API token in the macOS Keychain.
//!
//! The token used to sit in plaintext in `settings.json` inside the workflow's
//! data directory. The Keychain is encrypted at rest and access-controlled by
//! macOS, and — unlike Alfred's workflow configuration sheet — a token stored
//! here is not carried along when the workflow is exported or synced.
//!
//! This shells out to `security(1)` rather than linking Security.framework, to
//! avoid adding a dependency for two calls.

use std::process::Command;

use crate::AlfredError;

/// Account name for the Keychain item. The *service* is the workflow's bundle
/// id, so a token cannot be confused with another workflow's.
const ACCOUNT: &str = "pinboard-api-token";

fn service() -> Result<String, AlfredError> {
    std::env::var("alfred_workflow_bundleid").map_err(|_| AlfredError::MissingBundleId)
}

/// Read the token. `None` means "no item stored", which is not an error — it is
/// simply the state before the user has run `pa`.
pub fn get() -> Option<String> {
    let service = service().ok()?;
    let output = Command::new("security")
        .args(["find-generic-password", "-s", &service, "-a", ACCOUNT, "-w"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let token = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

/// Store the token, replacing any existing one.
pub fn set(token: &str) -> Result<(), AlfredError> {
    let service = service()?;
    let output = Command::new("security")
        .args([
            "add-generic-password",
            // Update in place instead of failing when an item already exists.
            "-U",
            "-s",
            &service,
            "-a",
            ACCOUNT,
            "-D",
            "application password",
            "-j",
            "Pinboard API token used by the Alfred Pinboard workflow",
            "-w",
            token,
        ])
        .output()
        .map_err(|e| AlfredError::KeychainError(e.to_string()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(AlfredError::KeychainError(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}
