-- global setting entries for various modules
DEFINE TABLE global_setting SCHEMAFULL;
DEFINE FIELD name ON global_setting TYPE string;
DEFINE FIELD val ON global_setting TYPE any;
DEFINE FIELD module ON global_setting TYPE option<record<module>> DEFAULT none;
DEFINE FIELD description ON global_setting TYPE option<string> DEFAULT none;

DEFINE INDEX global_settings_name_unique ON global_setting
    FIELDS name UNIQUE;

DEFINE INDEX global_settings_module ON global_setting
    FIELDS module;

DEFINE TABLE module SCHEMAFULL;
DEFINE FIELD enabled ON module TYPE bool DEFAULT false;

-- gets the record IDs for global settings that are associated with the given module
DEFINE FUNCTION fn::get_module_global_settings($mod: record<module>) -> array<record<global_setting>> {
    SELECT VALUE id FROM global_setting WHERE module = $mod
};

-- gets the names and values of all global settings associated with the given module
DEFINE FUNCTION fn::get_module_global_settings_full($mod: record<module>) -> array<object> {
    SELECT name, val FROM global_setting WHERE module = $mod
};

-- sets the value of a global setting
DEFINE FUNCTION fn::set_global_setting($name: string, $val: any) {
    UPDATE type::record("global_setting", $name) SET name = $name, val = $val;
};

-- gets the value of a global setting
DEFINE FUNCTION fn::get_global_setting($name: string) -> any {
    SELECT VALUE val FROM global_setting WHERE name = $name LIMIT 1
};

-- account setting definitions contain the info about the settings
-- as well as the default values. the API is similar to global_setting.
DEFINE TABLE account_setting_definition SCHEMAFULL;
DEFINE FIELD name ON account_setting_definition TYPE string;
DEFINE FIELD default_val ON account_setting_definition TYPE any;
DEFINE FIELD module ON account_setting_definition TYPE option<record<module>> DEFAULT none;
DEFINE FIELD description ON account_setting_definition TYPE option<string> DEFAULT none;

DEFINE INDEX account_settings_definition_name_unique ON account_setting_definition
    FIELDS name UNIQUE;

DEFINE INDEX account_settings_definition_module ON account_settings_definition
    FIELDS module;

-- gets the record IDs for setting definitions that are associated with the given module
DEFINE FUNCTION fn::get_module_setting_definitions($mod: record<module>) -> array<record<global_setting>> {
    SELECT VALUE id FROM account_setting_definition WHERE module = $mod
};

-- gets the names and values of all setting definitions associated with the given module
DEFINE FUNCTION fn::get_module_setting_definitions_full($mod: record<module>) -> array<object> {
    SELECT name, val FROM account_setting_definition WHERE module = $mod
};

-- sets the default value of an account setting
DEFINE FUNCTION fn::set_account_default_setting($name: string, $val: any) {
    UPDATE type::record("account_setting_definition", $name) SET name = $name, default_val = $val;
};

-- gets the default value of an account setting
DEFINE FUNCTION fn::get_account_default_setting($name: string) -> any {
    SELECT VALUE default_val FROM account_setting_definition WHERE name = $name LIMIT 1
};

-- account settings are per-account setting instances.
DEFINE TABLE account_setting SCHEMAFULL;
DEFINE FIELD definition ON account_setting TYPE record<account_setting_definition>;
DEFINE FIELD name ON account_setting TYPE string
    VALUE type::record("account_setting_definition", definition).name;
DEFINE FIELD val ON account_setting TYPE any
    DEFAULT type::record("account_setting_definition", definition).default_val;
DEFINE FIELD module ON account_setting TYPE option<record<module>>
    VALUE type::record("account_setting_definition", definition).module;
DEFINE FIELD description ON account_setting TYPE option<string>
    VALUE type::record("account_setting_definition", definition).description;
DEFINE FIELD account ON account_setting TYPE record<account>;

DEFINE INDEX account_settings_name_account_unique ON account_setting
    FIELDS name, account UNIQUE;

DEFINE INDEX account_settings_module ON account_setting
    FIELDS module;

DEFINE INDEX account_settings_def ON account_setting
    FIELDS definition;

DEFINE INDEX account_settings_account ON account_setting
    FIELDS account;

DEFINE FUNCTION fn::get_account_setting($acc: record<account>, $name: string) -> any {
    SELECT VALUE val FROM account_setting WHERE account = $acc AND name = $name LIMIT 1
};

DEFINE FUNCTION fn::set_account_setting($acc: record<account>, $name: string, $val: any) {
    UPDATE account_setting SET val = $val WHERE account = $acc AND name = $name;
};

-- initializes account settings with default values
DEFINE FUNCTION fn::init_account_settings($acc: record<account>) {
    FOR $def IN (SELECT VALUE id FROM account_setting_definition) {
        UPSERT account_setting
            SET
                definition = $def,
                account = $acc
            WHERE
                account = $acc
                AND definition = $def;
    }
};

-- gets the setting ids associated with an account
DEFINE FUNCTION fn::get_account_settings($acc: record<account>) -> array<record<account_setting>> {
    SELECT VALUE id FROM account_setting WHERE account = $acc
};

-- gets full setting objects associated with an account
DEFINE FUNCTION fn::get_account_settings_full($acc: record<account>) -> array<object> {
    SELECT * FROM account_setting WHERE account = $acc
};