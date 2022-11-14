mod app;
use app::App;

fn main() {
    use std::env;

    let app = App::create(env::args().collect());

    if app.is_err() {
        return;
    }

    let app = app.unwrap();

    if app.get_options().is_empty() {
        app.list_files();
    } else {
        app.apply_touch();
    }
}
