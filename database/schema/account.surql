DEFINE TABLE role SCHEMAFULL;
DEFINE FIELD perms ON role TYPE int;
DEFINE FIELD vis ON role TYPE "Visible" | "Hidden";
DEFINE FIELD display_name ON role TYPE option<string> DEFAULT none;
DEFINE FIELD display_color ON role TYPE option<string> DEFAULT none;

DEFINE TABLE account SCHEMAFULL;
DEFINE FIELD username ON account TYPE string
    ASSERT string::matches($value, "[a-z0-9]+");
DEFINE FIELD display_name ON account TYPE string;
DEFINE FIELD email ON account TYPE option<string> DEFAULT none
    ASSERT $value == none OR string::is_email($value);
DEFINE FIELD passwd_hash ON account TYPE string;
DEFINE FIELD avatar_url ON account TYPE string; -- TODO default to abstract art or something
DEFINE FIELD game_id ON account TYPE option<string> DEFAULT none;
DEFINE FIELD roles ON account TYPE array<record<role>> DEFAULT [];
DEFINE FIELD require_2fa ON account TYPE bool
    VALUE (
        (SELECT VALUE val FROM global_setting:force_2fa) OR
        email != none OR
        (
            game_id != none AND
            (SELECT VALUE val FROM global_setting:game_can_authenticate)
        )
    );

DEFINE INDEX account_username_unique ON account
    FIELDS username UNIQUE;

DEFINE INDEX account_game_id_unique ON account
    FIELDS game_id UNIQUE;

DEFINE INDEX account_roles ON account
    FIELDS roles;

DEFINE FUNCTION fn::account_fetch_roles($acc: record<account>) -> array<role> {
    SELECT * FROM $acc.roles
};

DEFINE FUNCTION fn::fetch_public_account($acc: record<account>) -> object {
    SELECT * OMIT passwd_hash, email FROM type::record('account', $acc)
};

DEFINE FUNCTION fn::get_account_2fa_methods($acc: record<account>) -> object {
    RETURN {
        email: $acc.email != none,
        game: $acc.game_id != none AND (SELECT VALUE val FROM global_setting:game_can_authenticate)
    };
}