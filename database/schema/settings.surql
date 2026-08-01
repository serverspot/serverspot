-- global setting entries for various modules
DEFINE TABLE global_setting SCHEMAFULL;
DEFINE FIELD name ON global_setting TYPE string;
DEFINE FIELD val ON global_setting TYPE any;
DEFINE FIELD module ON global_setting TYPE option<record<module>> DEFAULT none;

DEFINE INDEX global_settings_name_unique ON global_setting
    FIELDS name UNIQUE;

DEFINE INDEX global_settings_module ON global_setting
    FIELDS module;

DEFINE TABLE module SCHEMAFULL;
DEFINE FIELD enabled ON module TYPE bool DEFAULT false;

-- gets the record IDs for global settings that are associated with the given module
DEFINE FUNCTION fn::get_module_settings($mod: record<module>) -> array<record<global_setting>> {
    SELECT id FROM global_setting WHERE module = $mod
};

-- gets the names and values of all global settings associated with the given module
DEFINE FUNCTION fn::get_module_settings_full($mod: record<module>) -> array<object> {
    SELECT name, val FROM global_setting WHERE module = $mod
};

-- sets the value of a global setting
DEFINE FUNCTION fn::set_global_setting($name: string, $val: any) {
    UPDATE global_setting SET val = $val WHERE name = $name;
};

DEFINE FUNCTION fn::get_global_setting($name: string) -> any {
    SELECT val FROM global_setting WHERE name = $name LIMIT 1
};