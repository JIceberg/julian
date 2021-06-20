/// Models a query to the anilist API

pub use crate::utils::{
    Date, Int, Season
};

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Clone, Copy, Debug)]
pub enum Sort {
    Score,
    Popularity,
    Favorites,
    Episodes,
}

#[derive(Debug)]
pub struct MediaQuery {
    id: Int,
    start_date: Date,
    status: String,
    genres: Vec<String>,
    season: Season,
    score: Int,
    title: String,
    popularity: Int,
    favorites: Int,
    season_year: Int,
}

impl MediaQuery {
    pub fn new(
        title: String,
        id: Int,
        start_date: Date,
        status: String,
        genres: Vec<String>,
        score: Int,
        popularity: Int,
        favorites: Int,
        season: Season,
        season_year: Int
    ) -> Self {
        Self {
            id,
            start_date,
            status,
            genres,
            season,
            score,
            title,
            popularity,
            favorites,
            season_year
        }
    }
}

impl Serialize for MediaQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Anime", 10)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("startDate", &self.start_date.as_fuzzy())?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("averageScore", &self.score)?;
        s.serialize_field("popularity", &self.popularity)?;
        s.serialize_field("favorites", &self.favorites)?;
        s.serialize_field("season", &self.season)?;
        s.serialize_field("seasonYear", &self.season_year)?;
        let mut genre_str: String = "Genres: [ ".to_string();
        for i in 0..self.genres.len() {
            genre_str.push_str(self.genres[i].as_str());
            if i != self.genres.len() - 1 {
                genre_str.push_str(", ");
            }
        }
        genre_str.push_str(" ]");
        s.serialize_field("genres", &genre_str)?;
        s.end()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PageSize {
    Small = 10,
    Medium = 25,
    Large = 50,
}

use serde_json::{json, Value};
use reqwest::Client;

pub struct MediaRequest {
    sort: Sort,
    page: Int,
    page_size: PageSize,
}

const QUERY: &str = "
query($sort: [MediaSort], $page: Int, $perPage: Int) {
    Page(page: $page, perPage: $perPage) {
        media(sort: $sort, type: ANIME) {
            id
            popularity
            favourites
            title {
                romaji
                english
            }
            startDate {
                year
                month
                day
            }
            status
            averageScore
            season
            seasonYear
            genres
        }
    }
}
";

impl MediaRequest {
    pub fn new(
        sort: Sort,
        page: Int,
        page_size: PageSize
    ) -> Self {
        Self {
            sort,
            page,
            page_size
        }
    }

    pub async fn post(&self) -> Vec<MediaQuery> {
        let client = Client::new();

        let sorting_str = match self.sort {
            Sort::Episodes => "EPISODES_DESC",
            Sort::Favorites => "FAVOURITES_DESC",
            Sort::Popularity => "POPULARITY_DESC",
            Sort::Score => "SCORE_DESC"
        };

        let json = json!(
            {
                "query": QUERY,
                "variables": {
                    "sort": [sorting_str],
                    "page": self.page,
                    "perPage": self.page_size as Int
                }
            }
        );

        let resp = client.post("https://graphql.anilist.co/")
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .body(json.to_string())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await;

        let result: Value = serde_json::from_str(
            &resp.unwrap()
        ).unwrap();
        self.unwrap_value(result)
    }

    fn unwrap_value(&self, val: Value) -> Vec<MediaQuery> {
        let anime = val["data"]["Page"]["media"]
                    .as_array()
                    .unwrap();

        let mut page: Vec<MediaQuery> = vec![];

        for item in anime {
            let media: MediaQuery;

            let title = match item["title"]["english"].as_str() {
                Some(eng) => eng,
                None => match item["title"]["romaji"].as_str() {
                    Some(rom) => rom,
                    None => "TITLE MISSING"
                }
            };
            let id = item["id"].as_u64().unwrap() as Int;

            let score = item["averageScore"].as_u64().unwrap() as Int;
            let popularity = item["popularity"].as_u64().unwrap() as Int;
            let favorites = item["favourites"].as_u64().unwrap() as Int;

            let status = match item["status"].as_str() {
                Some(stat) => stat,
                None => "Unknown Status"
            }.to_string();

            let season_str = match item["season"].as_str() {
                Some(szn) => szn,
                None => "NONE"
            };
            let season: Season = match season_str {
                "WINTER" => Season::Winter,
                "SPRING" => Season::Spring,
                "SUMMER" => Season::Summer,
                "FALL" => Season::Fall,
                "NONE" => Season::None,
                _ => panic!("Somehow there were 5 seasons this year")
            };

            let start_year = match item["startDate"]["year"].as_u64() {
                Some(yr) => yr as Int,
                None => 0
            };
            let start_month = match item["startDate"]["month"].as_u64() {
                Some(month) => month as Int,
                None => 0
            };
            let start_day = match item["startDate"]["day"].as_u64() {
                Some(day) => day as Int,
                None => 0
            };
            let start_date = Date::new(
                start_year,
                start_month,
                start_day
            );
            let season_year = match item["seasonYear"].as_u64() {
                Some(yr) => yr as Int,
                None => start_date.get_year()
            };

            let genre_strs = item["genres"].as_array().unwrap();
            let mut genres: Vec<String> = Vec::new();
            for genre_str in genre_strs {
                genres.push(
                    genre_str
                        .as_str()
                        .unwrap()
                        .to_string()
                );
            }

            media = MediaQuery::new(
                title.to_string(),
                id,
                start_date,
                status,
                genres,
                score,
                popularity,
                favorites,
                season,
                season_year
            );
            page.push(media);
        }

        page
    }
}
