// use std::collections::HashMap;

use serde_json::json;

use crate::player_character::chargen_data::ChargenData;


pub async fn get_chargen_data(
    base_url: String,
    api_key: String,
) -> ChargenData {

    println!("Getting chargen data from {}", &base_url);
    let client = reqwest::Client::new();

    let map = json!({
        "api_key": api_key,
     });

    let chargen_data_result = client.post(base_url.clone() + &"/_api/chargen-data-get")
    .json(&map)
    .send()
    .await;

    let chargen_data = chargen_data_result.unwrap()
    .text()
    .await
    .unwrap();


    // block_on(chargen_data);
    println!("Completed downloading chargen data from {}, decoding json (strlen {})", &base_url, chargen_data.len());


    let data = serde_json::from_str(&chargen_data).unwrap();

    println!("...decoding json complete");

    return data;
}