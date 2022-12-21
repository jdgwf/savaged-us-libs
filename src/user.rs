use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use chrono_tz::Tz;

use crate::public_user_info::PublicUserInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub activated: bool,
    pub api_key: String,
    pub banned: bool,
    pub banned_by: u32,
    #[serde(default)]
    pub banned_on: Option<DateTime<Utc>>,
    pub banned_reason: String,
    pub created_by: u32,
    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
    pub default_username: bool,
    pub deleted: bool,
    pub deleted_by: u32,
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,
    pub discord_id: String,
    pub email: String,
    pub first_name: String,
    pub group_ids: Vec<u32>,
    pub hidden_banners: String, // JSON string
    pub id: u32,
    pub image_url: String,
    pub is_ace: bool,
    pub is_admin: bool,
    pub is_developer: bool,
    pub is_premium: bool,
    pub last_name: String,
    // pub last_seen_browser: String,
    pub last_seen_ip: String,
    #[serde(default)]
    pub last_seen_on: Option<DateTime<Utc>>,
    pub lc_wildcard_reason: String,
    pub login_tokens: Vec<LoginToken>,
    pub notes: String,
    pub notify_email: bool,
    pub number_years: u32,
    pub partner_id: u32,
    // pub password: String,
    pub paypal_payment_id: String,
    pub premium_expires: Option<DateTime<Utc>>,
    pub profile_image: String,
    #[serde(default)]
    pub reset_password_expire: Option<DateTime<Utc>>,
    // pub reset_password_link: String,
    // pub saves: Vec<Saves>,

    pub share_bio: String,
    pub share_display_name: String,
    pub share_show_profile_image: bool,
    pub show_user_page: bool,
    pub theme_css: String,
    pub timezone: String,
    pub turn_off_advance_limits: bool,
    pub twitter: String,
    pub updated_by: u32,
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,
    pub username: String,
    pub version_of: u32,
    pub zombie: bool,

    #[serde(default)]
    pub unread_notifications: u32,
    #[serde(default)]
    pub zombie_on: Option<DateTime<Utc>>,
}

impl Default for User {
    fn default() -> Self {
        User{
            activated: false,
            api_key: "".to_owned(),
            banned: false,
            banned_by: 0,
            banned_on: None,
            banned_reason: "".to_owned(),
            created_by: 0,
            created_on: Some(chrono::offset::Utc::now()),
            default_username: false,
            deleted: false,
            deleted_by: 0,
            deleted_on: None,
            discord_id: "".to_owned(),
            email: "".to_owned(),
            first_name: "".to_owned(),
            group_ids: Vec::new(),  // JSON "".to_owned()
            hidden_banners: "".to_owned(), // JSON "".to_owned()
            id: 0,
            is_ace: false,
            is_admin: false,
            is_developer: false,
            is_premium: false,
            last_name: "".to_owned(),
            // last_seen_browser: "".to_owned(),
            last_seen_ip: "".to_owned(),
            last_seen_on: None,
            lc_wildcard_reason: "".to_owned(),
            login_tokens: Vec::new(),
            notes: "".to_owned(),
            notify_email: false,
            image_url: "".to_owned(),
            number_years: 0,
            partner_id: 0,
            // password: "".to_owned(),
            paypal_payment_id: "".to_owned(),
            premium_expires: None,
            profile_image: "".to_owned(),
            reset_password_expire: None,
            // reset_password_link: "".to_owned(),
            // saves: Vec<Saves>,

            share_bio: "".to_owned(),
            share_display_name: "".to_owned(),
            share_show_profile_image: false,
            show_user_page: false,
            theme_css: "".to_owned(),
            timezone: "".to_owned(),
            turn_off_advance_limits: false,
            twitter: "".to_owned(),
            updated_by: 0,
            updated_on: None,
            username: "".to_owned(),
            version_of: 0,
            zombie: false,
            unread_notifications: 0,
            zombie_on: None,
        }
    }

}
impl User {

    pub fn get_name(
        &self,
    ) -> String {
        let export_name = self.first_name.to_owned() + &" ".to_owned() + &self.last_name.to_owned();
        return ( export_name ).trim().to_owned()
    }

    pub fn get_image(
        &self,
        base_url: &str,
    ) -> String {
        if self.profile_image.is_empty() {
            if self.is_admin {
                return base_url.to_owned() + &"/images/king.jpg".to_owned()  ;
            } else if self.is_developer {
                return base_url.to_owned() + &"/images/bishop.jpg".to_owned()  ;
            } else if self.is_premium {
                return base_url.to_owned() + &"/images/knight.jpg".to_owned()  ;
            } else {
                return base_url.to_owned() + &"/images/pawn.jpg".to_owned()  ;
            }
        } else {
            // return base_url.to_owned() + &self.image_url.to_owned()  ;

            let now = chrono::offset::Utc::now();
            let mut image_timestamp = now.timestamp();


            match self.updated_on {
                Some( date ) => {
                    image_timestamp = date.timestamp();
                }
                None => {

                }
            }
            let image_url = base_url.to_owned() + &format!("/data-images/users/{}.{}?u={}", self.id, self.profile_image, image_timestamp );
            return image_url.to_owned();
        }
    }

    pub fn get_user_url(
        &self,
    ) -> String {
        let export_name = "https://savaged.us/u/".to_owned() + &self.username.to_owned();
        return ( export_name ).trim().to_owned()
    }

    pub fn format_date(
        &self,
        incoming_date: DateTime<Utc>,
    ) -> String {
        let tz: Tz = self.timezone.parse().unwrap();
        let shifted = incoming_date.with_timezone(&tz);

        return shifted.format("%m/%d/%Y %H:%M:%S").to_string();
    }

    pub fn format_time(
        &self,
        incoming_date: DateTime<Utc>,
        with_seconds: bool,
        with_am_pm: bool,
        with_timezone: bool,
    ) -> String {
        let tz: Tz = self.timezone.parse().unwrap();
        let shifted = incoming_date.with_timezone(&tz);

        let mut format_string ="%H:%M".to_owned();

        if with_am_pm {
            format_string ="%l:%M".to_owned();
        }

        if with_seconds {
            format_string = format_string + ":%S";
        }

        if with_am_pm {
            format_string = format_string + " %p";
        }

        if with_timezone {
            format_string = format_string + " %Z";
        }

        return shifted.format(format_string.as_str() ).to_string();

    }

    pub fn format_datetime(
        &self,
        incoming_date: DateTime<Utc>,
        with_seconds: bool,
        with_am_pm: bool,
        with_timezone: bool,
    ) -> String {
        let tz: Tz = self.timezone.parse().unwrap();
        let shifted = incoming_date.with_timezone(&tz);

        let mut format_string ="%m/%d/%Y %H:%M".to_owned();

        if with_am_pm {
            format_string ="%m/%d/%Y %l:%M".to_owned();
        }

        if with_seconds {
            format_string = format_string + ":%S";
        }

        if with_am_pm {
            format_string = format_string + " %p";
        }

        if with_timezone {
            format_string = format_string + " %Z";
        }

        return shifted.format(format_string.as_str() ).to_string();

    }

    pub fn has_premium_access(&self) -> bool {
        if self.is_admin || self.is_developer || self.is_premium {
            return true;
        }
        return false;
    }


    pub fn get_real_name(&self) -> String {
        let mut real_name = "".to_string();

        if !self.first_name.is_empty() {
            real_name = self.first_name.to_owned();
        }
        if !self.last_name.is_empty() {
            real_name = real_name + &" ".to_owned() + &self.first_name.to_owned();
        }

        if real_name.is_empty() {
            return self.username.to_owned();
        } else {
            return real_name.trim().to_owned()
        }
    }

    pub fn has_admin_access(&self) -> bool {
        if self.is_admin  {
            return true;
        }
        return false;
    }
    pub fn has_developer_access(&self) -> bool {
        if self.is_admin || self.is_developer {
            return true;
        }
        return false;
    }

    pub fn get_display_name(&self) -> String {
        if self.share_display_name.is_empty() {
            self.share_display_name.to_owned()
        } else {
            self.username.to_owned()
        }
    }

    pub fn get_public_info(&self) -> PublicUserInfo {
        PublicUserInfo {
            username: self.username.to_owned(),
            name: self.get_display_name(),
            twitter: self.twitter.to_owned(),
            image: self.image_url.to_owned(),
            r#type: "".to_owned(),
            page: "".to_owned(),
            banned: false,
            banned_reason: "".to_owned(),
            show_user_page: self.show_user_page,
            user_page_url: "".to_owned(),
            bio: Vec::new(),
            shares: Vec::new(),
            id: self.id,
            room_id: "".to_owned(),

            shared_saves: Vec::new(),
        }
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoginTokenResult {
    pub success: bool,
    pub active_notifications: u32,
    pub user : User,
    pub user_groups: Vec<UserGroup>,
    pub login_token: String,
    #[serde(default)]
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    pub registered: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserUpdateResult {
    pub success: bool,
    pub password_changed: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserGroup {
    pub id: u32,
    pub name: String,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoginToken {
    pub last_seen: DateTime<Utc>,
    pub registered: DateTime<Utc>,
    pub friendly_name: String,
    pub browser: String,
    pub token: String,
    pub last_seen_ip: String,
}