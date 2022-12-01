
pub async fn get_chargen_data( api_key: String ) -> String {

    println!("Getting chargen data from savaged.us");
    let params = [("apikey", &api_key ) ];
    let client = reqwest::Client::new();
    let chargen_data = client.post("https://savaged.us/_api/chargen-data")
    // let chargen_data = client.post("http://localhost:5001/_api/chargen-data")
    .form(&params)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    // block_on(chargen_data);
    println!("Completed downloading chargen data from savaged.us");

    chargen_data
}