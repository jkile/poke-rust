use reqwest::{ Client, Url, Response, Error};



pub async fn get_pokemon() -> Result<Response, Error> {
    let client = Client::new();
    let response = client.get("https://pokeapi.co/api/v2/pokemon/gastly/").send().await?.text().await?;

    println!("{:?}", response);
    Ok(response)
}



