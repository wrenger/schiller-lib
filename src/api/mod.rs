use gdnative::prelude::*;

mod error;
pub use error::*;

mod project;
mod date;
mod debug;
mod mailer;
mod marc21;

use project::Project;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Project>();
    handle.add_class::<date::Date>();
    handle.add_class::<mailer::Mailer>();
    handle.add_class::<marc21::Marc21>();
}

// Macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
