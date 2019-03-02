use warp::{Filter, path};

mod score;

use score::ScoreService;

fn main() {
    let hello = path!(String)
        .map(|code| {
            let config = match ScoreService::new() {
                Some(c) => c,
                None => return String::from("Error"),
            };

            match config.scores(code) {
                Some(s) => s,
                None => String::from("No matches found today."),
            }
        });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030));
}
