use crate::{public_user_info::PublicUserInfo, player_character::armor::Armor};

pub trait RowEditData {
    fn id(&self) -> u32;
    fn name(&self) -> &'static str;
    fn summary(&self) -> &'static str;

    fn created_by_obj(&self) -> &'static Option<PublicUserInfo>;
    fn updated_by_obj(&self) -> &'static Option<PublicUserInfo>;
    fn deleted_by_obj(&self) -> &'static Option<PublicUserInfo>;
}

impl RowEditData for Armor {
    fn id(&self) -> u32 {
        self.id
    }
    fn name(&self) -> &'static str{
        let tmp = self.get_name();
        tmp.to_owned().as_str()
    }
    fn summary(&self) -> &'static str{
        &self.get_summary()
    }

    fn created_by_obj(&self) -> &'static Option<PublicUserInfo> {
        &self.created_by_obj
    }
    fn updated_by_obj(&self) -> &'static Option<PublicUserInfo> {
        &self.updated_by_obj
    }
    fn deleted_by_obj(&self) -> &'static Option<PublicUserInfo> {
        &self.deleted_by_obj
    }
}