//! Gravatar avatar URLs from email addresses (SHA-256).

use sha2::{Digest, Sha256};

/// Placeholder signed-in user until auth lands.
pub const CURRENT_USER_EMAIL: &str = "admin@serverspot.app";

/// Build a Gravatar image URL for `email`.
///
/// Email is trimmed and lowercased per Gravatar's hashing rules.
/// `size` is the requested pixel size (`s=`). Unknown emails fall back to
/// a unique identicon (`d=identicon`).
pub fn gravatar_url(email: &str, size: u32) -> String {
    let normalized = email.trim().to_ascii_lowercase();
    let digest = Sha256::digest(normalized.as_bytes());
    let hash = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();

    format!("https://www.gravatar.com/avatar/{hash}?s={size}&d=identicon&r=g")
}
