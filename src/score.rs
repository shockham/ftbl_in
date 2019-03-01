use log::error;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

const BASE_URL: &'static str = "https://api.football-data.org/v2/competitions/";

#[derive(Debug, Deserialize)]
pub struct ScoreService {
    api_key: String,
}

impl ScoreService {
    pub fn new() -> Option<ScoreService> {
        let toml_str = match File::open("./ftbl.toml") {
            Ok(mut f) => {
                let mut toml_str = String::new();
                let _ = f.read_to_string(&mut toml_str);
                toml_str
            }
            Err(e) => {
                error!("ScoreService: {}", e);
                return None;
            }
        };

        Some(toml::from_str(toml_str.as_str()).unwrap())
    }

    fn get_scores_json(&self) -> Result<serde_json::Value, Box<std::error::Error>> {
        let comp_url = format!("{}{}/matches", BASE_URL, "PL");
        let client = reqwest::Client::new();
        let resp = client
            .get(comp_url.as_str())
            .header("X-Auth-Token", self.api_key.clone())
            .query(&[("dateFrom", "2019-02-27"), ("dateTo", "2019-02-27")])
            .send()?
            .json()?;
        Ok(resp)
    }

    pub fn scores(&self) -> String {
        let json = self.get_scores_json().unwrap();

        let mut all_scores = String::new();

        for game in json["matches"].as_array().unwrap() {
            let home_team_name = &game["homeTeam"]["name"].as_str().unwrap();
            let away_team_name = &game["awayTeam"]["name"].as_str().unwrap();
            let home_team_score = &game["score"]["fullTime"]["homeTeam"];
            let away_team_score = &game["score"]["fullTime"]["awayTeam"];

            let score_string = format!(
                "{} {} - {} {}\n",
                home_team_name, home_team_score, away_team_score, away_team_name
            );

            all_scores.push_str(score_string.as_str());
        }

        all_scores
    }
}
