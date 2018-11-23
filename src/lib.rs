#[macro_use]

extern crate samp_sdk;
extern crate scraper;
extern crate minihttp;

mod plugin;
mod natives;

use plugin::PawnScraper;

new_plugin!(PawnScraper with process_tick);

