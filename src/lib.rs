#[macro_use]
extern crate samp_sdk;
extern crate scraper;

mod plugin;
mod natives;

use plugin::PawnScraper;

new_plugin!(PawnScraper);

