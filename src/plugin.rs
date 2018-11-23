use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use scraper::{Html,Selector};
use natives::Natives;
use minihttp::request::Request;
use std::sync::mpsc::{Sender,Receiver,channel};


define_native!(parse_document,document:String);
define_native!(parse_document_by_response,id:usize);
define_native!(parse_selector,string:String);
define_native!(http_request,url:String);
define_native!(http_request_threaded,playerid:usize,callback:String,url:String);
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
	pub request_send: Option<Sender<(usize, String, String)>>,
	pub response_recv: Option<Receiver<(usize, String, String)>>,
	pub amx_list :Vec<usize>,
	
}
impl PawnScraper{
	pub fn load(&mut self) -> bool {
		let (mut send_response, mut rcv_response):(Sender<(usize, String, String)>,Receiver<(usize, String, String)>) = channel();
		let (mut send_request,mut rcv_request):(Sender<(usize, String, String)>,Receiver<(usize, String, String)>) = channel();
		//*&mut self.response_recv.unwrap() = rcv_response;
		//*&self.request_send.unwrap() = send_request.clone();
		let mut send_response_clone = send_response.clone();
		for (playerid,url,callback) in rcv_request.iter() {
			
			std::thread::spawn(move || {
				
				match Request::new(&url){
					Ok(mut http) =>{
						match http.get().send(){
							Ok(res) => {
								let body = res.text();
								send_response_clone.send((playerid, callback, body)).unwrap();
								
							}
							Err(err) =>{
								log!("Http error {:?} for url {:?}",err,url);
							}
						}
					}
					Err(err) =>{
						log!("Url parse error {:?}",err);
					}
				}
				
			});
		}
		log!("Plugin Loaded!");
		return true;
	}

	pub fn unload(&self) {
		log!("Plugin Unloaded!");
	}

	pub fn amx_load(mut self, amx: &mut AMX) -> Cell {
		self.amx_list.push(amx.amx as usize);
		let natives = natives!{
			"ParseHtmlDocument" => parse_document,
			"ResponseParseHtml" => parse_document_by_response,
			"ParseSelector" => parse_selector,
			"HttpGet" => http_request,
			"HttpGetThreaded" => http_request,
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

	pub fn amx_unload(&mut self, amx: &mut AMX) -> Cell {
		let raw = amx.amx as usize;
		let index = self.amx_list.iter().position(|x| *x == raw).unwrap().clone();
		self.amx_list.remove(index);
		AMX_ERR_NONE
	}

	pub fn process_tick(&mut self) {
		for (playerid, callback, body) in  self.response_recv.unwrap().try_iter() {
			self.response_cache.insert(self.response_context_id,body);
			self.response_context_id += 1;
			let responseid = self.response_context_id as Cell -1;
			for amx in &self.amx_list{
				let amx_pointer = unsafe { std::mem::transmute(amx) };
				let amx = AMX::new(amx_pointer);
				let index = amx.find_public(&callback).unwrap();
				if index<0{
					continue;
				}
				amx.push(responseid);
				amx.push(playerid);
				amx.exec(index);
			}
		}
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
			request_send:None,
			response_recv:None,
			amx_list:Vec::new(),
		}
	}
}