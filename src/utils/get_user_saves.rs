use crate::save_db_row::SaveDBRow;
// use std::collections::HashMap;
use serde_json::json;


pub async fn get_user_saves(
    base_url: String,
    api_key: String,
) -> Vec<SaveDBRow> {

    println!("Getting user saves data from {}", &base_url);

    let map = json!({
        "api_key": api_key,
     });

    let client = reqwest::Client::new();

    let user_saves_result = client.post(base_url.clone() + &"/_api/saves/get")
    .json(&map)
    .send()
    .await;

    let user_saves = user_saves_result.unwrap()
    .text()
    .await
    .unwrap();

    // block_on(user_saves);
    println!("Completed downloading user saves data from {}", &base_url);
    println!("{}", &user_saves );

    let user_saves_return:Vec<SaveDBRow> = serde_json::from_str(&user_saves).unwrap();
    return user_saves_return;
}

