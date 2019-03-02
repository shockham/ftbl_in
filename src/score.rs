use ansi_term::{Colour, Style};
use chrono::prelude::*;
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

        match toml::from_str(toml_str.as_str()) {
            Ok(t) => Some(t),
            Err(e) => {
                error!("ScoreService: {}", e);
                return None;
            }
        }
    }

    fn get_scores_json(
        &self,
        comp_code: String,
    ) -> Result<serde_json::Value, Box<std::error::Error>> {
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

    pub fn scores(&self, comp_code: String) -> Option<String> {
        let json = match self.get_scores_json(comp_code) {
            Ok(j) => j,
            Err(e) => {
                error!("ScoreService: {}", e);
                return None;
            }
        };

        let mut all_scores = String::new();

        let competition_name = json["competition"]["name"].as_str()?;
        let styled_competition_name = Style::new()
            .on(Colour::Fixed(8))
            .bold()
            .paint(format!("{}:", competition_name));
        all_scores.push_str(format!("\n{}\n", styled_competition_name).as_str());

        for game in json["matches"].as_array()? {
            let home_team_name = &game["homeTeam"]["name"].as_str()?;
            let away_team_name = &game["awayTeam"]["name"].as_str()?;
            let home_team_score = &game["score"]["fullTime"]["homeTeam"];
            let away_team_score = &game["score"]["fullTime"]["awayTeam"];

            let score_string = {
                if !home_team_score.is_null() {
                    let styled_match_score = Style::new()
                        .fg(Colour::Black)
                        .on(Colour::White)
                        .bold()
                        .paint(format!(" {} - {} ", home_team_score, away_team_score));

                    format!(
                        "{} {} {}\n",
                        home_team_name, styled_match_score, away_team_name
                    )
                } else {
                    let match_time = &game["utcDate"].as_str()?;
                    let match_status = match DateTime::parse_from_rfc3339(match_time) {
                        Ok(time) => time.format("%H:%M").to_string(),
                        Err(e) => {
                            error!("ScoreService: {}", e);
                            let status_str = &game["status"].as_str()?;
                            String::from(*status_str)
                        }
                    };

                    let styled_match_status = Style::new()
                        .fg(Colour::Black)
                        .on(Colour::White)
                        .bold()
                        .paint(format!(" {} ", match_status));

                    format!(
                        "{} {} {}\n",
                        home_team_name, styled_match_status, away_team_name
                    )
                }
            };

            all_scores.push_str(score_string.as_str());
        }

        if all_scores.is_empty() {
            return None;
        }

        all_scores.push_str("\n");

        Some(all_scores)
    }

    fn get_competitions_json(&self) -> Result<serde_json::Value, Box<std::error::Error>> {
        let client = reqwest::Client::new();
        let resp = client.get(BASE_URL).send()?.json()?;
        Ok(resp)
    }

    pub fn competitions(&self) -> Option<String> {
        let json = match self.get_competitions_json() {
            Ok(j) => j,
            Err(e) => {
                error!("ScoreService: {}", e);
                return None;
            }
        };

        let mut all_competitions =
            String::from("\nUsage:\n curl ftbl.in/<competition>\n\nCompetitions:\n");

        for competition in json["competitions"].as_array()? {
            let competition_code = &competition["code"];

            if !competition_code.is_null() {
                let competition_name = &competition["name"].as_str()?;
                all_competitions.push_str(
                    format!("{} : {}\n", competition_code.as_str()?, competition_name).as_str(),
                );
            }
        }

        Some(all_competitions)
    }
}
