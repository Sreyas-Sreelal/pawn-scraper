#[macro_use]

extern crate samp_sdk;
extern crate scraper;
extern crate minihttp;

#[macro_use]
mod macros;
mod plugin;
mod natives;
mod internals;

use plugin::PawnScraper;

new_plugin!(PawnScraper with process_tick);

