
pub async fn get_user_saves( api_key: String ) -> String {

    println!("Getting user saves data from savaged.us");
    let params = [("apikey", &api_key ) ];
    let client = reqwest::Client::new();
    let user_saves = client.post("https://savaged.us/_api/auth/get-saves")
    // let chargen_data = client.post("http://localhost:5001/get_user_saves")
    .form(&params)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    // block_on(chargen_data);
    println!("Completed downloading user saves data from savaged.us");

    user_saves
}

