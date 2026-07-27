use sha2::{Digest, Sha256};

pub const CURRENT_USER_EMAIL: &str = "admin@serverspot.app";
pub const CURRENT_USER_NAME: &str = "Charlie Admin";

/// SHA-256 of `admin@serverspot.app` (trimmed, lowercased).
const CURRENT_USER_HASH: &str = "93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269";

pub const CURRENT_USER_AVATAR_64: &str =
    "https://www.gravatar.com/avatar/93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269?s=64&d=identicon&r=g";
pub const CURRENT_USER_AVATAR_144: &str =
    "https://www.gravatar.com/avatar/93cd5fcda5d91c37acb38aabdf13c9b5bb7abb70d08ed55797a462d68de24269?s=144&d=identicon&r=g";

const HEX: &[u8; 16] = b"0123456789abcdef";

fn push_u32(buf: &mut String, mut value: u32) {
    if value == 0 {
        buf.push('0');
        return;
    }
    let mut digits = [0u8; 10];
    let mut n = 0;
    while value > 0 {
        digits[n] = b'0' + (value % 10) as u8;
        value /= 10;
        n += 1;
    }
    while n > 0 {
        n -= 1;
        buf.push(digits[n] as char);
    }
}

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
        let mut url = String::with_capacity(96);
        url.push_str("https://www.gravatar.com/avatar/");
        url.push_str(CURRENT_USER_HASH);
        url.push_str("?s=");
        push_u32(&mut url, size);
        url.push_str("&d=identicon&r=g");
        return url;
    }

    let normalized = email.trim().to_ascii_lowercase();
    let digest = Sha256::digest(normalized.as_bytes());

    let mut url = String::with_capacity(96);
    url.push_str("https://www.gravatar.com/avatar/");
    for byte in digest {
        url.push(HEX[(byte >> 4) as usize] as char);
        url.push(HEX[(byte & 0x0f) as usize] as char);
    }
    url.push_str("?s=");
    push_u32(&mut url, size);
    url.push_str("&d=identicon&r=g");
    url
}
