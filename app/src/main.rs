use api;



#[tokio::main]
async fn main() {
    // let name = String::from("mewtwo");
    // let pokemon = api::get_pokemon(&name).await;
    // match pokemon {
    //     Ok(v) => println!("{:?}", v.json::<serde_json::Value>().await),
    //     Err(e) => println!("{:?}", e)
    // };
    let pokemon = api::get_pokemon("gastly").await;
    println!("{:?}", pokemon)
}