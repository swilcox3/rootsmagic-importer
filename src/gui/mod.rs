use crate::utils::*;
use fltk::{app, prelude::*, window::Window, button, input};
use crate::importers::wikitree;

macro_rules! rc {
	($inner: expr) => {
		std::rc::Rc::from(std::cell::RefCell::from($inner))
	};
}

pub fn run() -> Result<(), ImportError> {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut window = Window::new(100, 100, 400, 300, "Rootsmagic Importer");
    let mut search_button = button::Button::new(160, 200, 80, 40, "Search");
    let mut first_search = rc!(input::Input::new(100, 100, 80, 40, "First Name"));
    let mut last_search = rc!(input::Input::new(100, 150, 80, 40, "Last Name"));
    search_button.set_callback({
	let first = first_search.clone();
	let last = last_search.clone();
	move |_| {
		let first_name = first.borrow().value();
		let last_name = last.borrow().value();
		let mut search = wikitree::WikiTreeSearchPersonParams::default();
		if !first_name.is_empty() {
			search.FirstName = Some(first_name);
		}
		if !last_name.is_empty() {
			search.LastName = Some(last_name);
		}
		let results = wikitree::search_person(search).unwrap();
		println!("{:?}", results);
	}
    });
    window.end();
    window.show();
    app.run()?;
    Ok(())
}
