extern crate yew;
extern crate z1rtracker;

use yew::prelude::*;
use z1rtracker::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
