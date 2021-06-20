use julian::query::{MediaRequest, PageSize, Sort, MediaQuery};

use tokio;
extern crate csv;

use std::error::Error;

fn write_to_csv(anime_list: Vec<MediaQuery>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("anime.csv").unwrap();

    for anime in anime_list {
        wtr.serialize(anime)?;
    }

    wtr.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let req = MediaRequest::new(Sort::Popularity, 3, PageSize::Large);
    let my_list = req.post().await;

    match write_to_csv(my_list) {
        Ok(_) => {},
        Err(err) => panic!("Error: {}", err)
    };
}
