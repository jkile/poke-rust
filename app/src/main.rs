use api;



#[tokio::main]
async fn main() {


    let gastly = api::get_pokemon().await.text().await;
    println!("{:?}", gastly);
    

}