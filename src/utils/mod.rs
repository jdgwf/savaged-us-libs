use uuid::{Uuid};
pub mod get_dice_value;
pub mod get_chargen_data;
pub mod get_user_saves;
pub mod date_formatting;
pub mod success_return;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::prelude::*;
use serde::de::{self, Unexpected};
use chrono_tz::Tz;

pub fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    // match u32::deserialize(deserializer).as_str() {
    //     "0" => return Ok(false),
    //     "1" => return Ok(true),
    //     "false" => return Ok(false),
    //     "true" => return Ok(true),
    //     other => return Err(de::Error::invalid_value(
    //         Unexpected::Str(other),
    //         &"zero or one",
    //     )),
    // }
    match String::deserialize(deserializer) {

        Ok( val ) => {
            match val.as_ref() {
                "0" => return Ok(false),
                "1" => return Ok(true),
                "false" => return Ok(false),
                "true" => return Ok(true),
                other => return Err(de::Error::invalid_value(
                    Unexpected::Str(other),
                    &"zero or one",
                )),
            }
        }

        Err(err) => {
            return Ok(false);
        }

    }

}

pub fn float_to_int<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    match f32::deserialize(deserializer)? {

        val => {
            let moo = val.round() as u32;

            return Ok( moo );
        },
    }

}

// pub fn uuid_to_string<'se, S>(serializer: S) -> Result<String, D::Error>
// where
//     S: Serializer<'se>,
// {
//     match Uuid::serialize(serializer)? {

//         val => {
//             let moo = val.to_string();

//             return Ok( moo );
//         },
//     }

// }
// fn uuid_to_string<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     s.to_string()
// }

// pub fn string_to_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     match String::deserialize(deserializer)? {

//         val => {
//             let moo = Uuid::parse_str( &val ).unwrap();

//             return Ok( moo );
//         },

//     }

// }