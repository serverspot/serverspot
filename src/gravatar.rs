use sha2::{Digest, Sha256};

pub fn gravatar_url(email: &str, size: u32) -> String {
    let normalized = email.trim().to_ascii_lowercase();
    let digest = Sha256::digest(normalized.as_bytes());
    format!("https://www.gravatar.com/avatar/{digest:x}?s={size}&d=identicon&r=g")
}

#[cfg(test)]
mod tests {
    use super::gravatar_url;

    #[test]
    fn hashes_email_for_url() {
        assert_eq!(
            gravatar_url("admin@serverspot.app", 64),
            "https://www.gravatar.com/avatar/93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269?s=64&d=identicon&r=g"
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
