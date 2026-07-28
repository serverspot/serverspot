use sha2::{Digest, Sha256};

pub const CURRENT_USER_EMAIL: &str = "admin@serverspot.app";
pub const CURRENT_USER_NAME: &str = "Charlie Admin";

/// SHA-256 of `admin@serverspot.app` (trimmed, lowercased).
const CURRENT_USER_HASH: &str = "93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269";

pub const CURRENT_USER_AVATAR_64: &str =
    "https://www.gravatar.com/avatar/93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269?s=64&d=identicon&r=g";
pub const CURRENT_USER_AVATAR_144: &str =
    "https://www.gravatar.com/avatar/93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269?s=144&d=identicon&r=g";

pub fn current_user_avatar(size: u32) -> Option<&'static str> {
    match size {
        64 => Some(CURRENT_USER_AVATAR_64),
        144 => Some(CURRENT_USER_AVATAR_144),
        _ => None,
    }
}

pub fn gravatar_url(email: &str, size: u32) -> String {
    if email == CURRENT_USER_EMAIL {
        if let Some(url) = current_user_avatar(size) {
            return url.to_string();
        }
        return format!("https://www.gravatar.com/avatar/{CURRENT_USER_HASH}?s={size}&d=identicon&r=g");
    }

    let normalized = email.trim().to_ascii_lowercase();
    let digest = Sha256::digest(normalized.as_bytes());
    format!("https://www.gravatar.com/avatar/{digest:x}?s={size}&d=identicon&r=g")
}

#[cfg(test)]
mod tests {
    use super::{gravatar_url, CURRENT_USER_AVATAR_64, CURRENT_USER_HASH};

    #[test]
    fn returns_prebuilt_current_user_avatar_for_known_size() {
        assert_eq!(
            gravatar_url("admin@serverspot.app", 64),
            CURRENT_USER_AVATAR_64
        );
    }

    #[test]
    fn builds_current_user_avatar_with_format_for_other_sizes() {
        assert_eq!(
            gravatar_url("admin@serverspot.app", 32),
            format!("https://www.gravatar.com/avatar/{CURRENT_USER_HASH}?s=32&d=identicon&r=g")
        );
    }

    #[test]
    fn normalizes_email_before_hashing() {
        assert_eq!(
            gravatar_url("  USER@Example.COM  ", 80),
            "https://www.gravatar.com/avatar/b4c9a289323b21a01c3e940f150eb9b8c542587f1abfd8f0e1cc1ffc5e475514?s=80&d=identicon&r=g"
        );
    }
}
