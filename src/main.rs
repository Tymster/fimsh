mod fish;
use dotenv::dotenv;
use fish::Fish;
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
    pub async fn fish_update(req: Request<Fishes>) -> tide::Result {
        let fish = find_fish(
            &req.state().collection,
            req.param("id").unwrap().parse::<String>().unwrap(),
        )
        .await
        .unwrap();
        println!("Fish : {:?}", fish);
        Ok("jerome".into())
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct FishUpdate {
    id: String,
    value: i32,
}
#[tokio::main]
async fn main() -> tide::Result<()> {
    // let url = format!(
    //     "mongodb+srv://user:{}@cluster.2qxtmxu.mongodb.net/?retryWrites=true&w=majority",
    //     std::env::var("PASSWORD").unwrap()
    // );
    let fishes = Fishes::connect().await?;
    Fish::grr(&fishes.collection).await?;
    dotenv().ok();
    let mut app = tide::with_state(Fishes::connect().await.unwrap());
    println!("New app made");
    app.at("/update/:id").post(Fishes::fish_update);
    app.at("/cdn/:id").get(handle_image);
    app.listen("127.0.0.1:8080").await.unwrap();
    Ok(())
}
async fn handle_image(req: Request<Fishes>) -> tide::Result {
    let id = req.param("id").unwrap();
    match std::path::Path::new(&format!("./images/{id}.png")).exists() {
        false => Ok("Not found".into()),
        _ => {
            let mut res = tide::Response::new(tide::StatusCode::Ok);
            res.set_body(
                tide::Body::from_file(format!("./images/{id}.jpg"))
                    .await
                    .unwrap(),
            );
            Ok(res.into())
        }
    }
}
async fn find_fish(
    collection: &mongodb::Collection<Document>,
    id: String,
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
