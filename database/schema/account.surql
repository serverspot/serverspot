DEFINE TABLE account SCHEMAFULL;
DEFINE FIELD username ON account TYPE string
    ASSERT string::is_ascii($value) AND string::is_lowercase($value);
DEFINE FIELD display_name ON account TYPE string;
DEFINE FIELD email ON account TYPE option<string> DEFAULT none
    ASSERT $value == none OR string::is_email($value);
DEFINE FIELD login_hash ON account TYPE string;
DEFINE FIELD avatar_url ON account TYPE string; -- TODO default to abstract art or something
DEFINE FIELD admin ON account TYPE bool DEFAULT false;
DEFINE FIELD game_id ON account TYPE option<string> DEFAULT none;

-- one-time passcode for verification. used for pretty much all 2fa methods.
DEFINE FIELD otp ON account TYPE option<string> DEFAULT none;
DEFINE FIELD otp_expires ON account TYPE option<timestamp> DEFAULT none;

DEFINE INDEX account_username_unqiue ON account
    FIELDS username UNIQUE;

DEFINE INDEX account_game_id_unique ON account
    FIELDS game_id UNIQUE;

-- generate a new otp for verification. it expires in one hour.
DEFINE FUNCTION fn::get_otp($acc: record<account>) -> string {
    LET $otp = rand::string(10);
    UPDATE $acc SET otp = $otp, otp_expires = time::now() + 1h;
    RETURN $otp;
};

-- check whether the given otp matches the user's otp and isn't expired.
DEFINE FUNCTION fn::validate_otp($acc: record<account>, $attempt: string) -> bool {
    LET $data = SELECT otp, otp_expires FROM $acc;
    RETURN
        $data.otp = $attempt AND
        $data.otp_expires >= time::now();
};