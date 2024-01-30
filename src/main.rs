

use fltk::{app, prelude::*, window::Window};

fn main() {
    let a = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "windmap");
    wind.end();
    wind.show();
    a.run().unwrap();
}