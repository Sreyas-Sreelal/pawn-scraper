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
	pub response_recv: Option<Receiver<(usize, String, String,bool)>>,
	pub amx_list :Vec<usize>,
}

impl PawnScraper{
	pub fn load(&mut self) -> bool {
		let (send_response, rcv_response) = channel();
		let (send_request, rcv_request) = channel();
		
		self.response_recv = Some(rcv_response);
		self.request_send = Some(send_request);
			
		std::thread::spawn(move || {
			for (playerid,callback,url) in rcv_request.iter() {
				match Request::new(&url){
					Ok(mut http) =>{
						match http.get().send(){
							Ok(res) => {
								let body = res.text();
								send_response.send((playerid, callback, body,true)).unwrap();
							}
							Err(_err) =>{
								send_response.send((playerid, callback, String::from(""),false)).unwrap();
								//log!("Http error {:?} for url {:?}",err,url);
							}
						}
					}
					Err(_err) =>{
						send_response.send((playerid, callback, String::from(""),false)).unwrap();
						//log!("Url parse error {:?} url is {:?}",err,url);
					}
				}
			}	
		});
		log!("PawnScraper loaded");
		return true;
	}
		

	pub fn unload(&self) {
		log!("Plugin Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		log!("amx is {:?}",amx.amx);
		self.amx_list.push(amx.amx as usize);
		let natives = natives!{
			"ParseHtmlDocument" => parse_document,
			"ResponseParseHtml" => parse_document_by_response,
			"ParseSelector" => parse_selector,
			"HttpGet" => http_request,
			"HttpGetThreaded" => http_request_threaded,
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
		for (playerid, callback, body,success) in  self.response_recv.as_ref().unwrap().try_iter() {
			let body = body.as_str();
			for amx in &self.amx_list{
				let amx = AMX::new(*amx as *mut _);
				match amx.find_public(&callback){
					Ok(index) =>{
						let mut responseid = -1;
						if success {
							self.response_cache.insert(self.response_context_id,String::from(body));
							self.response_context_id += 1;
							responseid = self.response_context_id as Cell -1;
						}
						amx.push(responseid).unwrap();
						amx.push(playerid).unwrap();
						amx.exec(index).unwrap();
					}
					Err(err) =>{
						log!("Error finding callback {:?}",err);
						continue;
					}
				};
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