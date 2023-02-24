use async_std::process::Command;
use mongodb::bson::doc;
use mongodb::bson::Document;
use rand;
use rand::Rng;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Fish {
    pub id: u32,
    pub rating: i32,
    pub name: String,
}
impl Fish {
    pub async fn load(collection: &mongodb::Collection<Document>) -> std::io::Result<()> {
        let names: Vec<String> = std::fs::read_to_string("names.txt")
            .unwrap()
            .split("\n")
            .map(|f| f.to_string())
            .collect();
        collection.delete_many(doc! {}, None).await.unwrap();
        Command::new("mkdir").arg("temp").spawn().unwrap();
        for (i, img) in std::fs::read_dir("./images").unwrap().enumerate() {
            std::fs::rename(
                img.unwrap().path().display().to_string(),
                format!("./temp/{i}.jpg"),
            )?;
            collection
                .insert_one(
                    doc! {
                        "id" : i as u32,
                        "rating" : 100,
                        "name" : names[rand::thread_rng().gen_range(0..names.len())].to_owned() +  " " + &names[rand::thread_rng().gen_range(0..names.len())],
                    },
                    None,
                )
                .await
                .unwrap();
        }
        for n in std::fs::read_dir("./temp").unwrap() {
            std::fs::copy(
                &n.as_ref().unwrap().path(),
                format!("./images/{}", &n.unwrap().file_name().to_str().unwrap()),
            )?;
        }
        Command::new("rm").arg("-r").arg("temp").spawn().unwrap();
        Ok(())
    }
}
