use crate::setting::Setting;
// use chrono::prelude::*;

impl Setting {
    pub fn import_json(&mut self, import_json_string: String) {
        // let import_data: JSONSettingDefinition = serde_json::from_str(&import_json_string).unwrap();

        // self.name = import_data.name.clone();
    }

    pub fn import(&mut self, import_data: Setting) {
        self.name = import_data.name.clone();
    }
}
