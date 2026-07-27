
use sha2::{Digest, Sha256};

pub const CURRENT_USER_EMAIL: &str = "admin@serverspot.app";

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

pub fn gravatar_url(email: &str, size: u32) -> String {
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
