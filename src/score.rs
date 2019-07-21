use ansi_term::{Colour, Style};
use chrono::prelude::*;
use futures::Future;
use log::error;
use serde_derive::Deserialize;
use toml;

const BASE_URL: &str = "https://api.football-data.org/v2/competitions/";
const CONFIG: &str = include_str!("../ftbl.toml");

#[derive(Debug, Deserialize)]
pub struct ScoreRepo {
    api_key: String,
}

impl ScoreRepo {
    pub fn new() -> Option<ScoreRepo> {
        match toml::from_str(CONFIG) {
            Ok(t) => Some(t),
            Err(e) => {
                error!("ScoreRepo: {}", e);
                None
            }
        }
    }

    pub fn get_scores(
        &self,
        comp_code: String,
    ) -> impl Future<Item = reqwest::r#async::Response, Error = reqwest::Error> {
        let comp_url = format!("{}{}/matches", BASE_URL, comp_code);
        let todays_date = Utc::today().format("%Y-%m-%d").to_string();
        let client = reqwest::r#async::Client::new();

        client
            .get(comp_url.as_str())
            .header("X-Auth-Token", self.api_key.clone())
            .query(&[("dateFrom", todays_date.clone()), ("dateTo", todays_date)])
            .send()
    }

    pub fn get_competitions(
    ) -> impl Future<Item = reqwest::r#async::Response, Error = reqwest::Error> {
        let client = reqwest::r#async::Client::new();
        client.get(BASE_URL).send()
    }
}

pub struct ScoreView;

impl ScoreView {
    pub fn scores(json: serde_json::Value) -> Option<String> {
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

    pub fn competitions(json: serde_json::Value) -> Option<String> {
        let mut all_competitions =
            String::from("\nUsage:\n curl ftbl.in/<competition>\n\nCompetitions:\n");

        for competition in json["competitions"].as_array()? {
            let competition_code = &competition["code"];

            if !competition_code.is_null() {
                let competition_name = &competition["name"].as_str()?;
                let styled_competition_code = Style::new()
                    .fg(Colour::Black)
                    .on(Colour::White)
                    .bold()
                    .paint(competition_code.as_str()?);
                all_competitions.push_str(
                    format!("   {}: {}\n", styled_competition_code, competition_name).as_str(),
                );
            }
        }

        all_competitions.push_str("\n");

        Some(all_competitions)
    }
}
