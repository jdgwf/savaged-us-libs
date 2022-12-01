use crate::player_character::PlayerCharacter;
use crate::json_data::json_character_export::JSONCharacterExport;
use crate::setting::Setting;
use chrono::prelude::*;

use super::edge::Edge;
use super::hindrance::Hindrance;

impl PlayerCharacter {
    pub fn import_json(
        &mut self,
        import_json_string: String,
    ) {
        let import_data: JSONCharacterExport = serde_json::from_str(&import_json_string).unwrap();

        self.name = import_data.name.clone();
        self.attributes.selected_agility = import_data.attribute_assignments.agility;
        self.attributes.selected_smarts = import_data.attribute_assignments.smarts;
        self.attributes.selected_spirit = import_data.attribute_assignments.spirit;
        self.attributes.selected_strength = import_data.attribute_assignments.strength;
        self.attributes.selected_vigor = import_data.attribute_assignments.vigor;

        // self.setting = Setting::new_import(self.available_data.clone());

        self.setting.import( import_data.setting );
        // self.created_on = DateTime::from_utc(DateTime::parse_from_rfc3339( &import_data.created_on ).unwrap().naive_utc(), Utc);
        // self.updated_on = DateTime::from_utc(DateTime::parse_from_rfc3339( &import_data.updated_on ).unwrap().naive_utc(), Utc);
        // self.deleted_on = DateTime::from_utc(DateTime::parse_from_rfc3339( &import_data.deleted_on ).unwrap().naive_utc(), Utc);
        for def in import_data.edges {
            let mut edge = Edge::new();
            edge.import_from_id( def.id, &self.available_data );
            edge.import_vars( &def.edgeOptions );
            self.selected_edges.push(edge);
        }

        // for def in import_data.hindrances {
        //     let mut hindrance = Hindrance::new();
        //     hindrance.import( def.id, def, &self.available_data );
        //     self.selected_hindrances.push(hindrance);
        // }
    }
}