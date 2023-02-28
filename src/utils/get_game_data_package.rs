// use std::collections::HashMap;

use serde_json::json;

use crate::player_character::game_data_package::GameDataPackage;

pub async fn get_game_data_package(base_url: String, api_key: String) -> GameDataPackage {
    println!("Getting game data from {}", &base_url);
    let client = reqwest::Client::new();

    let map = json!({
       "api_key": api_key,
    });

    let game_data_result = client
        .post(base_url.clone() + &"/_api/game-data/get")
        .json(&map)
        .send()
        .await;

    let game_data = game_data_result.unwrap().text().await.unwrap();

    // block_on(game_data);
    println!(
        "Completed downloading game data from {}, decoding json (strlen {})",
        &base_url,
        game_data.len()
    );

    let data = serde_json::from_str(&game_data).unwrap();

    println!("...decoding json complete");

    return data;
}
