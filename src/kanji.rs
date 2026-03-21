use dioxus::prelude::{asset, Asset};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Kanji {
    #[serde(default)]
    id: i32,
    pub kanji: String,
    pub keyword: String,
    #[serde(default)]
    pub components: String,
    #[serde(default)]
    pub on_reading: String,
    #[serde(default)]
    pub kun_reading: String,
    #[serde(default)]
    pub jlpt: String
}

pub fn read_file(file: Asset) -> Result<Vec<Kanji>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(dioxus_asset_resolver::asset_path(&file)?)?;
    let mut kanjis: Vec<Kanji> = Vec::new();
    for result in rdr.deserialize() {
        let record: Kanji = result?;
        kanjis.push(record);
    }
    Ok(kanjis)
}