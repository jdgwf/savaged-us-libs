use crate::player_character::PlayerCharacter;

impl PlayerCharacter {
    pub fn export_html( &self ) -> String {
        let mut export_html: String = "".to_owned();

        export_html = export_html + "<h1>" + &self.name + "</h1>";

        export_html = export_html + "<strong>Attributes</strong>:";
        export_html = export_html + "&nbsp;Agility: " + &self.attributes.agility_hr();
        export_html = export_html + "&nbsp;Smarts: " + &self.attributes.smarts_hr();
        export_html = export_html + "&nbsp;Spirit: " + &self.attributes.spirit_hr();
        export_html = export_html + "&nbsp;Strength: " + &self.attributes.strength_hr();
        export_html = export_html + "&nbsp;Vigor: " + &self.attributes.vigor_hr();

        export_html
    }
}