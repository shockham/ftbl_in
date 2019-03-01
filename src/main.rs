use log::error;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    api_key: String,
}

fn handle_config() -> Option<Config> {
    let toml_str = match File::open("./ftbl.toml") {
        Ok(mut f) => {
            let mut toml_str = String::new();
            let _ = f.read_to_string(&mut toml_str);
            toml_str
        }
        Err(e) => {
            error!("Config: {}", e);
            return None;
        }
    };

    Some(toml::from_str(toml_str.as_str()).unwrap())
}

const SCORES_URL: &str = "https://api.football-data.org/v2/competitions/PL/matches";

fn get_scores(api_key: String) -> Result<serde_json::Value, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(SCORES_URL)
        .header("X-Auth-Token", api_key)
        .query(&[("dateFrom", "2019-02-27"), ("dateTo", "2019-02-27")])
        .send()?
        .json()?;
    Ok(resp)
}

fn format_scores(json: serde_json::Value) -> String {
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

    print!("{}", all_scores);

    all_scores
}

fn main() {
    let config = match handle_config() {
        Some(c) => c,
        None => return,
    };

    let scores_json = get_scores(config.api_key).unwrap();

    format_scores(scores_json);
}
