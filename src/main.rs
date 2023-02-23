mod fish;
use async_std::stream::StreamExt;
use dotenv::dotenv;
use fish::Fish;
use mongodb::bson::{from_document, Document};
use mongodb::{bson::doc, Client};
use tide::prelude::*;
use tide::Request;
use tokio;
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
    pub async fn fish_update(mut req: Request<Fishes>) -> tide::Result {
        let update: FishUpdate = req.body_json().await.unwrap();
        req.state()
            .collection
            .find_one_and_update(
                doc! {
                    "id" : req.param("id").unwrap().parse::<u32>().unwrap()
                },
                doc! {
                    "$inc" : {
                        "rating" : update.value
                    }
                },
                None,
            )
            .await
            .unwrap()
            .unwrap();
        Ok(Fishes::new(req).await.unwrap())
    }
    async fn new(req: Request<Fishes>) -> tide::Result {
        let x = req
            .state()
            .collection
            .aggregate(vec![doc! {"$sample": {"size": 1}}], None)
            .await
            .unwrap()
            .collect::<Vec<Result<_, mongodb::error::Error>>>()
            .await;
        let fish: Fish = from_document(x[0].clone().unwrap()).unwrap();
        Ok(serde_json::to_string(&fish).unwrap().into())
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct FishUpdate {
    value: i32,
}
#[tokio::main]
async fn main() -> tide::Result<()> {
    // let url = format!(
    //     "mongodb+srv://user:{}@cluster.2qxtmxu.mongodb.net/?retryWrites=true&w=majority",
    //     std::env::var("PASSWORD").unwrap()
    // );
    let fishes = Fishes::connect().await?;
    // Fish::grr(&fishes.collection).await?;
    dotenv().ok();
    let mut app = tide::with_state(fishes);
    println!("New app made");

    app.at("/update/:id").post(Fishes::fish_update);
    app.at("/cdn/:id").get(handle_image);
    app.at("/new").get(Fishes::new);
    app.listen("127.0.0.1:8080").await.unwrap();
    Ok(())
}
async fn handle_image(req: Request<Fishes>) -> tide::Result {
    let id = req.param("id").unwrap();
    match std::path::Path::new(&format!("./images/{id}.jpg")).exists() {
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
