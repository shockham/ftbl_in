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

            config.scores(code)
        });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030));
}
