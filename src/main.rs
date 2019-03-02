use warp::{path, Filter};

mod score;

use score::ScoreService;

fn main() {
    let help = warp::path::end().map(|| {
        let config = match ScoreService::new() {
            Some(c) => c,
            None => return String::from("Error"),
        };

        match config.competitions() {
            Some(s) => s,
            None => String::from("No matches found today."),
        }
    });

    let comps = path!(String).map(|code| {
        let config = match ScoreService::new() {
            Some(c) => c,
            None => return String::from("Error"),
        };

        match config.scores(code) {
            Some(s) => s,
            None => String::from("No matches found today."),
        }
    });

    let addr = if cfg!(debug_assertions) {
        ([127, 0, 0, 1], 3030)
    } else {
        ([0, 0, 0, 0], 80)
    };

    warp::serve(help.or(comps)).run(addr);
}
