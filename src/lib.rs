#[macro_use]

extern crate samp_sdk;
extern crate minihttp;
extern crate scraper;

#[macro_use]
mod macros;
mod internals;
mod natives;
mod plugin;

use plugin::PawnScraper;

new_plugin!(PawnScraper with process_tick);
