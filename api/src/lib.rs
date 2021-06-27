use reqwest::{ self };
use serde_json;


// pub async fn get_pokemon(name: &str) -> Result<Response> {
//     let mut url  = String::from("https://pokeapi.co/api/v2/pokemon/");
//     url.push_str(name);
//     let response = reqwest::get(url).await;
//   response
// }

pub async fn get_pokemon(name: &str) -> serde_json::Value {
    let mut url  = String::from("https://pokeapi.co/api/v2/pokemon/");
    url.push_str(name);
    if let Ok(response) = reqwest::get(url).await {
        response.json::<serde_json::Value>().await
    }
}