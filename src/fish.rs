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
        std::fs::create_dir("./images/temp")?;
        for (i, img) in std::fs::read_dir("./images").unwrap().skip(1).enumerate() {
            match img.as_ref().unwrap().file_type().unwrap().is_dir() {
                true => continue,
                false => {
                    std::fs::rename(
                        img.unwrap().path().display().to_string(),
                        format!("./images/temp/{i}.jpg"),
                    )?;
                    let rnd = || return rand::thread_rng().gen_range(0..names.len());
                    collection
                        .insert_one(
                            doc! {
                                "id" : i as u32,
                                "rating" : 100,
                                "name" : format!("{} {}" , names[rnd()] , names[rnd()]),
                            },
                            None,
                        )
                        .await
                        .unwrap();
                }
            }
        }
        for n in std::fs::read_dir("./images/temp").unwrap() {
            std::fs::copy(
                &n.as_ref().unwrap().path(),
                format!("./images/{}", &n.unwrap().file_name().to_str().unwrap()),
            )?;
        }
        std::fs::remove_dir_all("./images/temp")?;
        Ok(())
    }
}
