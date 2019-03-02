use log::error;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;
use chrono::prelude::*;
use ansi_term::Style;

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

    fn get_scores_json(&self, comp_code:String) -> Result<serde_json::Value, Box<std::error::Error>> {
        let comp_url = format!("{}{}/matches", BASE_URL, comp_code);
        let todays_date = Utc::today().format("%Y-%m-%d").to_string();
        let client = reqwest::Client::new();
        let resp = client
            .get(comp_url.as_str())
            .header("X-Auth-Token", self.api_key.clone())
            .query(&[("dateFrom", todays_date.clone()), ("dateTo", todays_date)])
            .send()?
            .json()?;
        Ok(resp)
    }

    pub fn scores(&self, comp_code:String) -> Option<String> {
        let json = match self.get_scores_json(comp_code) {
            Ok(j) => j,
            Err(e) => {
                error!("ScoreService: {}", e);
                return None;
            },
        };

        let mut all_scores = String::new();

        for game in json["matches"].as_array()? {
            let home_team_name = &game["homeTeam"]["name"].as_str()?;
            let away_team_name = &game["awayTeam"]["name"].as_str()?;
            let home_team_score = &game["score"]["fullTime"]["homeTeam"];
            let away_team_score = &game["score"]["fullTime"]["awayTeam"];

            let score_string = {
                if !home_team_score.is_null() {
                    format!(
                        "{} [{} - {}] {}\n",
                        home_team_name,
                        Style::new().bold().paint(format!("{}", home_team_score)),
                        Style::new().bold().paint(format!("{}", away_team_score)),
                        away_team_name
                    )
                } else {
                    let match_status = &game["status"].as_str()?;
                    format!(
                        "{} [{}] {}\n",
                        home_team_name, Style::new().bold().paint(*match_status), away_team_name
                    )
                }
            };

            all_scores.push_str(score_string.as_str());
        }

        if all_scores.is_empty() {
            return None;
        }

        Some(all_scores)
    }
}
