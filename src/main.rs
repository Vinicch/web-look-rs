mod cli;

use cli::{builder, lookup};

fn main() {
    let matches = builder::build_app().get_matches();

    lookup::lookup(matches);
}
