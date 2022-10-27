mod app;
use app::{App, print_usage};


fn main() {
    use std::env;

    let app = App::create(env::args().collect());

    if app.is_err() {
        print_usage();
        return;
    }

    let app = app.unwrap();

    if app.get_options().is_empty() {
        app.list_files();
    }

    app.apply_touch();
}
