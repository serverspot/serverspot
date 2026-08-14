-- axum_session_surreal support
DEFINE TABLE auth_session SCHEMAFULL;
DEFINE FIELD sessionstore ON auth_session TYPE string;
DEFINE FIELD sessionexpires ON auth_session TYPE int;
DEFINE FIELD sessionid ON auth_session TYPE uuid;

-- TODO index by account id which is in sessionstore json