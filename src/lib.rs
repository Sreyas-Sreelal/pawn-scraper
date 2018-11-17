#[macro_use]
extern crate samp_sdk;
extern crate scraper;
extern crate mio_httpc;
mod plugin;
mod natives;

use plugin::PawnScraper;

new_plugin!(PawnScraper);

