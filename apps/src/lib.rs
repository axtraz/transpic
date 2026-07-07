#![warn(clippy::all, rust_2018_idioms)]

rust_intl::load!(default = "en");

mod app;
pub use app::TemplateApp;
