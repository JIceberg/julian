/// Models a query to the anilist API

use crate::utils::{Date, Int,Season};

pub enum Sort {
    Score,
    Popularity,
    Favorites,
    Episodes,
}

pub struct MediaQuery {
    id: Int,
    id_mal: Int,
    start_date: Date,
    end_date: Date,
    genres: Vec<String>,
    season: Season,
    episodes: Int,
    score: Int,
    popularity: Int,
    favorites: Int,
    season_year: Int,
}

// impl MediaQuery {
//     pub fn new(
//         id: Int,
//         id_mal: Int,

//     ) -> Self {

//     }
// }