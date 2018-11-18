use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use scraper::{Html,Selector};
use natives::Natives;

define_native!(parse_document,document:String);
define_native!(parse_document_by_response,id:usize);
define_native!(parse_selector,string:String);
define_native!(http_request,url:String);
define_native!(get_nth_element_name,docid:usize,selectorid:usize,idx:usize,string:ref Cell,size:usize);
define_native!(get_nth_element_text,docid:usize,selectorid:usize,idx:usize,string:ref Cell,size:usize);
define_native!(get_nth_element_attr_value,docid:usize, selectorid:usize,idx:usize,attr:String,string:ref Cell,size:usize);
define_native!(delete_response_cache,id:usize);
define_native!(delete_html_instance,id:usize);
define_native!(delete_selector_instance,id:usize);

pub struct PawnScraper{
	pub html_instance: std::collections::HashMap<usize,Html>,
	pub selectors: std::collections::HashMap<usize,Selector>,
	pub response_cache: std::collections::HashMap<usize,String>,
	pub html_context_id: usize,
	pub selector_context_id: usize,
	pub response_context_id: usize,
}

impl PawnScraper{
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
			"ResponseParseHtml" => parse_document_by_response,
			"ParseSelector" => parse_selector,
			"HttpGet" => http_request,
			"GetNthElementName" => get_nth_element_name,
			"GetNthElementText" => get_nth_element_text,
			"GetNthElementAttrVal" => get_nth_element_attr_value,
			"DeleteHtml" => delete_html_instance,
			"DeleteSelector" => delete_selector_instance,
			"DeleteResponse" => delete_response_cache
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

impl Default for PawnScraper{
	fn default() -> Self {
		PawnScraper {
			html_instance: std::collections::HashMap::new(),
			selectors: std::collections::HashMap::new(),
			response_cache: std::collections::HashMap::new(),
			html_context_id: 0,
			selector_context_id: 0,
			response_context_id: 0,
		}
	}
}