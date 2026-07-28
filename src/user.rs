#[derive(Clone, PartialEq)]
pub struct CurrentUser {
    pub email: String,
    pub name: String,
    pub role: String,
}

/// Seeded until real auth lands. Swap this initializer without touching call sites.
pub fn placeholder_current_user() -> CurrentUser {
    CurrentUser {
        email: String::from("admin@serverspot.app"),
        name: String::from("Charlie Admin"),
        role: String::from("Owner"),
    }
}

#[cfg(test)]
mod tests {
    use super::placeholder_current_user;

    #[test]
    fn placeholder_has_identity_fields() {
        let user = placeholder_current_user();
        assert_eq!(user.email, "admin@serverspot.app");
        assert_eq!(user.name, "Charlie Admin");
        assert_eq!(user.role, "Owner");
    }
}
