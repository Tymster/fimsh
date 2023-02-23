mod fish;
use dotenv::dotenv;
use mongodb::bson::{from_document, Bson, Document};
use mongodb::{bson::doc, Client};
use tide::prelude::*;
use tide::Request;
use tokio;
use uuid;
#[derive(Clone)]
struct Fishes {
    collection: mongodb::Collection<Document>,
}
impl Fishes {
    pub async fn connect() -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();

        Ok(Self {
            collection: client.database("fish").collection::<Document>("fish"),
        })
    }
    pub async fn handle_update(req: Request<Fishes>) -> tide::Result {
        let fish = find_fish(
            &req.state().collection,
            req.param("id").unwrap().parse::<u32>().unwrap(),
        )
        .await
        .unwrap();
        println!("Fish : {:?}", fish);
        Ok("jerome".into())
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct FishUpdate {
    id: u32,
    value: i32,
}
#[tokio::main]
async fn main() -> tide::Result<()> {
    let fishes = Fishes::connect().await?;
    dotenv().ok();

    let mut app = tide::with_state(Fishes::connect().await.unwrap());
    println!("New app made");
    app.at("/update/:id").post(Fishes::handle_update);
    app.listen("127.0.0.1:8080").await.unwrap();
    // let url = format!(
    //     "mongodb+srv://user:{}@cluster.2qxtmxu.mongodb.net/?retryWrites=true&w=majority",
    //     std::env::var("PASSWORD").unwrap()
    // );
    Ok(())
}
async fn handle_update(req: Request<Fishes>) -> tide::Result {
    Ok("jerome".into())
}
async fn find_fish(
    collection: &mongodb::Collection<Document>,
    id: u32,
) -> Result<fish::Fish, mongodb::bson::de::Error> {
    Ok(from_document(
        collection
            .find_one(
                doc! {
                    "id" : id,
                },
                None,
            )
            .await
            .unwrap()
            .unwrap(),
    )
    .unwrap())
}

// let mut app = tide::new();
// app.at("/fish/update").post(handle_update);
// app.listen("127.0.0.1:8080").await?;
// Ok(())
