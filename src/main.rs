mod database;
mod importers;
#[macro_use]
mod utils;

use crate::utils::*;
use fltk::{app, button, input, prelude::*, window::Window};
use importers::ImportSource;

fn val_as_opt(input: &input::Input) -> Option<String> {
    let value = input.value();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn search_window(importer: Box<dyn ImportSource>) {
    let mut window = Window::new(100, 100, 400, 300, "Search Person");
    let mut search_button = button::Button::new(160, 200, 80, 40, "Search");
    let first_search = rc!(input::Input::new(100, 100, 80, 40, "First Name"));
    let last_search = rc!(input::Input::new(100, 150, 80, 40, "Last Name"));
    search_button.set_callback({
        let first = first_search.clone();
        let last = last_search.clone();
        move |_| {
            let search = importers::Search::new(val_as_opt(&first.borrow()), val_as_opt(&last.borrow()), None, None, None, None, importers::Gender::Unknown, None, None, None, None);
            let results = importer.search_person(search).unwrap();
            println!("{:?}", results);
        }
    });
    window.end();
    window.show();
}

fn run() -> Result<(), ImportError> {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut window = Window::new(100, 100, 400, 300, "Rootsmagic Importer");
    let mut search_button = button::Button::new(160, 200, 80, 40, "Search Wikitree");
    search_button.set_callback(|_|{
        let importer = importers::wikitree::WikiTreeImporter::new();
        search_window(Box::new(importer) as Box<dyn ImportSource>);
    });
    window.show();
    app.run()?;
    Ok(())
}

fn main() {
    run().unwrap();
}
