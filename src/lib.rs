#[macro_use]
extern crate samp_sdk;
extern crate scraper;
extern crate reqwest;

mod plugin;
mod natives;

use plugin::PawnScraper;

new_plugin!(PawnScraper);

