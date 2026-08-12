#[derive(Clone, PartialEq)]
pub struct CurrentUser {
    pub email: String,
    pub username: String,
    pub name: String,
    pub role: String,
    pub locale: String,
}

pub fn placeholder_current_user() -> CurrentUser {
    CurrentUser {
        email: String::from("admin@serverspot.app"),
        username: String::from("charlie"),
        name: String::from("Charlie Admin"),
        role: String::from("Owner"),
        locale: String::from(crate::i18n::DEFAULT_LOCALE),
    }
}
#[cfg(test)]
mod tests {
    use super::placeholder_current_user;

    #[test]
    fn placeholder_has_identity_fields() {
        let user = placeholder_current_user();
        assert_eq!(user.username, "charlie");
        assert_eq!(user.name, "Charlie Admin");
        assert_eq!(user.role, "Owner");
    }
}
