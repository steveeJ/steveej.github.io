#![recursion_limit = "256"]

use website::Model;

fn main() {
    web_logger::init();
    yew::start_app::<Model>();
}
