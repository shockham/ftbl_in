use warp::{Filter, path};

mod score;

use score::ScoreService;

fn main() {
    let hello = path!(String)
        .map(|name| {
            let config = match ScoreService::new() {
                Some(c) => c,
                None => return String::from("Error"),
            };

            config.scores()
        });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030));
}
