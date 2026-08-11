-- a global role object
DEFINE TABLE role SCHEMAFULL;
DEFINE FIELD perms ON role TYPE int;
DEFINE FIELD vis ON role TYPE "Visible" | "Hidden";
DEFINE FIELD display_name ON role TYPE option<string> DEFAULT none;
DEFINE FIELD display_color ON role TYPE option<string> DEFAULT none;

-- a connection for a specific account
DEFINE TABLE account_connection SCHEMAFULL;
DEFINE FIELD connection_info ON account_connection TYPE string;
DEFINE FIELD verified ON account_connection TYPE bool DEFAULT false;
DEFINE FIELD account ON account_connection TYPE record<account>;
DEFINE FIELD ty ON account_connection TYPE "email" | "game";

DEFINE INDEX account_connection_connection_info_unique ON account_connection
    FIELDS connection_info UNIQUE;

DEFINE INDEX account_connection_account ON account_connection
    FIELDS account;

DEFINE INDEX account_connection_account_ty_unique ON account_connection
    FIELDS account, ty UNIQUE;

-- the main account table
DEFINE TABLE account SCHEMAFULL;
DEFINE FIELD username ON account TYPE string
    ASSERT string::matches($value, "[a-z0-9]+");
DEFINE FIELD display_name ON account TYPE string;
DEFINE FIELD passwd_hash ON account TYPE string;
DEFINE FIELD avatar_url ON account TYPE string; -- TODO default to abstract art or something
DEFINE FIELD roles ON account TYPE array<record<role>> DEFAULT [];
DEFINE FIELD require_2fa ON account TYPE bool
    VALUE (
        (global_setting:force_2fa.val OR fn::get_account_setting($parent, "2fa_enabled")) AND
        (
            email != none OR
            (
                game_id != none AND
                global_setting:game_can_authenticate.val
            )
        )
    );
DEFINE FIELD game_id ON account TYPE option<string>
    VALUE fn::get_account_connection($parent, "game").?.connection_info;
DEFINE FIELD game_id_verified ON account TYPE option<string>
    VALUE fn::get_account_connection($parent, "game").?.verified;
DEFINE FIELD email ON account TYPE option<string>
    VALUE fn::get_account_connection($parent, "email").?.connection_info;
DEFINE FIELD email_verified ON account TYPE option<string>
    VALUE fn::get_account_connection($parent, "game").?.verified;

DEFINE INDEX account_username_unique ON account
    FIELDS username UNIQUE;

DEFINE INDEX account_game_id_unique ON account
    FIELDS game_id UNIQUE;

DEFINE INDEX account_roles ON account
    FIELDS roles.*;

DEFINE FUNCTION fn::account_fetch_roles($acc: record<account>) -> array<object> {
    SELECT * FROM $acc.roles
};

DEFINE FUNCTION fn::fetch_public_account($acc: record<account>) -> object {
    SELECT * OMIT passwd_hash, connections FROM type::record('account', $acc)
};

DEFINE FUNCTION fn::get_account_connection($acc: record<account>, $ty: string) -> option<record<account_connection>> {
    SELECT VALUE id FROM account_connection WHERE account = $acc AND ty = $ty LIMIT 1
};

DEFINE FUNCTION fn::list_account_connections($acc: record<account>) -> array<record<account_connection>> {
    SELECT VALUE id FROM account_connection WHERE account = $acc
};