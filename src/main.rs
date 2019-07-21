use futures::Future;
use warp::{path, Filter};
use reqwest;

mod score;

use score::{ScoreRepo, ScoreView};

fn main() {
    env_logger::init();

    let help = path::end()
        .and_then(|| {
            ScoreRepo::get_competitions()
                .map(|resp| resp)
                .map_err(|err| warp::reject::custom(err))
        })
        .and_then(|mut resp: reqwest::r#async::Response| {
            resp.json()
                .map(|json| json)
                .map_err(|err| warp::reject::custom(err))
        })
        .map(|json| match ScoreView::competitions(json) {
            Some(s) => s,
            None => String::from("No matches found today."),
        });

    let comps = path::param()
        .and_then(|code: String| {
            let config = match ScoreRepo::new() {
                Some(c) => c,
                None => panic!("Error"),
            };

            config
                .get_scores(code)
                .map(|resp| resp)
                .map_err(|err| warp::reject::custom(err))
        })
        .and_then(|mut resp: reqwest::r#async::Response| {
            resp.json()
                .map(|json| json)
                .map_err(|err| warp::reject::custom(err))
        })
        .map(|json| match ScoreView::scores(json) {
            Some(s) => s,
            None => String::from("No matches found today."),
        });

    let addr = if cfg!(debug_assertions) {
        ([127, 0, 0, 1], 3030)
    } else {
        ([0, 0, 0, 0], 80)
    };

    warp::serve(help.or(comps)).run(addr);
}
