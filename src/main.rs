mod app;
use app::{App, print_usage};

use crate::app::OptEnum;
fn main() {
    use std::env;

    let app = App::create(env::args().collect());

    if let Err(x) = app {
        print_usage();
        return;
    }

    let app = app.unwrap();

    if app.GetOptions().is_empty() {
        app.list_files();
    }

    println!("{:?}", app);


}
