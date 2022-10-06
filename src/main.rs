mod app;
use app::{App, print_usage};
fn main() {
    use std::env;

    let app = App::create(env::args().collect());

    if let Err(x) = app {
        print_usage();
        return;
    }
    println!("{:?}", app);


}
