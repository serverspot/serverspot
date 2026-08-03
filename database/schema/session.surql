-- axum_session_surreal support
DEFINE TABLE auth_session SCHEMAFULL;
DEFINE FIELD sessionstore ON auth_session TYPE string;
DEFINE FIELD sessionexpires ON auth_session TYPE int;
DEFINE FIELD sessionid ON auth_session TYPE uuid;

-- custom data attached to axum_session_surreal
DEFINE FIELD accountid ON auth_session TYPE option<record<account>> DEFAULT none;

DEFINE INDEX auth_session_accountid ON auth_session
    FIELDS accountid;

-- a session is anonymous if accountid is none.
DEFINE FUNCTION fn::get_anonymous_sessions() -> array<record<auth_session>> {
    SELECT id FROM auth_session WHERE accountid = none
}