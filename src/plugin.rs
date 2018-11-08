use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use scraper::{Html,Selector};
use natives::Natives;

define_native!(parse_document,document:String);
define_native!(parse_selector,string:String);

pub struct PawnScraper<'a,'b>{
	pub html_instance: Vec<Html>,
	pub selectors: Vec<Selector>,
	pub element_iterators: Vec<scraper::html::Select<'a,'b>>,
}

impl<'a,'b> PawnScraper <'a,'b>{
	pub fn load(&self) -> bool {
		log!("Plugin Loaded!");
		return true;
	}

	pub fn unload(&self) {
		log!("Plugin Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		let natives = natives!{
			"ParseHtmlDocument" => parse_document,
			"ParseSelector" => parse_selector
		};

		match amx.register(&natives) {
			Ok(_) => log!("Natives are successful loaded"),
			Err(err) => log!("Whoops, there is an error {:?}", err),
		}

		AMX_ERR_NONE
	}

	pub fn amx_unload(&self, _: &mut AMX) -> Cell {
		AMX_ERR_NONE
	}

}

impl<'a,'b> Default for PawnScraper<'a,'b> {
	fn default() -> Self {
		PawnScraper {
			html_instance: Vec::new(),
			selectors: Vec::new(),
			element_iterators: Vec::new(),
			
		}
	}
}