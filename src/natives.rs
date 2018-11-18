use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};
use scraper::{Html,Selector};
use minihttp::request::Request;
use std::thread;
use std::sync::{Arc,Mutex};


pub trait Natives {
	fn parse_document(&mut self,_:&AMX,document:String) -> AmxResult<Cell>;
	fn parse_document_by_response(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>;
	fn parse_selector(&mut self,_:&AMX,string:String) -> AmxResult<Cell>;
	fn http_request(&mut self,_:&AMX,url:String) -> AmxResult<Cell>;
	fn http_request_threaded(&self,amx:&AMX,callback:String,url:String) -> AmxResult<Cell>;
	fn get_nth_element_name(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>;
	fn get_nth_element_text(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>;
	fn get_nth_element_attr_value(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,attr:String,string:&mut Cell,size:usize) -> AmxResult<Cell>;
	fn delete_html_instance(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>;
	fn delete_selector_instance(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>;
	fn delete_response_cache(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>;	
}

impl Natives for super::PawnScraper{
	fn parse_document(&mut self,_:&AMX,document:String) -> AmxResult<Cell> {
		let parsed_data = Html::parse_document(&document);
		self.html_instance.insert(self.html_context_id,parsed_data);
		self.html_context_id += 1;
		Ok(self.html_context_id  as Cell -1)
	}

	fn parse_document_by_response(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>{
		if id > *self.response_context_id.lock().unwrap() {
			Ok(-1)
		}else{
			let bind = self.response_cache.lock().unwrap();
			let response_data = bind.get(&id);
			if response_data == None{
				Ok(-1)
			}else{
				let parsed_data = Html::parse_document(&response_data.unwrap());
				self.html_instance.insert(self.html_context_id,parsed_data);
				self.html_context_id += 1;
				Ok(self.html_context_id  as Cell -1)
			}
		}
	}

	fn parse_selector(&mut self,_:&AMX,string:String) -> AmxResult<Cell> {
		match Selector::parse(&string){
			Ok(selector) => {
				self.selectors.insert(self.selector_context_id,selector);
				self.selector_context_id += 1;
				Ok(self.selector_context_id as Cell -1)
			}
			Err(err) =>{
				log!("Failed parsing selector {:?}",err);
				Ok(-1)
			}
		}
	}

	fn get_nth_element_text(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>{
		if !self.html_instance.contains_key(&docid) || !self.selectors.contains_key(&selectorid){
			log!("Invalid html instances passed docid {:?},selectorid {:?}",docid,selectorid);
			Ok(-1)
		}else{
			let html = &self.html_instance.get(&docid).unwrap();
			let selector = &self.selectors.get(&selectorid).unwrap();
			let nth_element = html.select(selector).nth(idx);
			if nth_element == None{
				Ok(0)
			}else{
				let element_text_iter = nth_element.unwrap().text();
				let mut full_text:String = String::new();
				for i in element_text_iter{
					full_text += i;
				}
				let text_encoded = samp_sdk::cp1251::encode(&full_text).unwrap();
				set_string!(text_encoded,string,size);
				Ok(1)
			}
		}
	}

	fn get_nth_element_name(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>{
		if !self.html_instance.contains_key(&docid) || !self.selectors.contains_key(&selectorid){
			log!("Invalid html instances passed docid {:?},selectorid {:?}",docid,selectorid);
			Ok(-1)
		}else{
			let html = &self.html_instance.get(&docid).unwrap();
			let selector = &self.selectors.get(&selectorid).unwrap();
			let nth_element = html.select(selector).nth(idx);
			
			if nth_element == None{
				Ok(0)
			}else{
				let element_name = nth_element.unwrap().value().name();
				let name_encoded = samp_sdk::cp1251::encode(element_name).unwrap();
				set_string!(name_encoded,string,size);
				Ok(1)
			}
		}
	}
	
	fn get_nth_element_attr_value(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,attr:String,string:&mut Cell,size:usize) -> AmxResult<Cell>{
		if !self.html_instance.contains_key(&docid) || !self.selectors.contains_key(&selectorid){
			log!("Invalid html instances passed docid {:?},selectorid {:?}",docid,selectorid);
			Ok(-1)
		}else{
			let html = &self.html_instance.get(&docid).unwrap();
			let selector = &self.selectors.get(&selectorid).unwrap();
			let nth_element = html.select(selector).nth(idx);
			if nth_element == None{
				Ok(0)
			}else{
				let attr_value = nth_element.unwrap().value().attr(&attr);
				if attr_value == None{
					Ok(-2)
				}else{
					let attr_encoded = samp_sdk::cp1251::encode(attr_value.unwrap()).unwrap();
					set_string!(attr_encoded,string,size);
					Ok(1)
				}
			}
		}
	}

	fn http_request(&mut self,_:&AMX,url:String) -> AmxResult<Cell>{
		match Request::new(&url){
			Ok(mut http) =>{
				match http.get().send(){
					Ok(res) => {
						let body = res.text();
						self.response_cache.lock().unwrap().insert(*self.response_context_id.lock().unwrap(),body);
						*self.response_context_id.lock().unwrap() += 1;
						Ok(*self.response_context_id.lock().unwrap() as Cell -1)
					}
					Err(err) =>{
						log!("Http error {:?}",err);
						Ok(-1)
					}
				}
			}
			Err(err) =>{
				log!("Url parse error {:?}",err);
				Ok(-1)
			}
		}
	}


	fn http_request_threaded(&self,amx:&AMX,callback:String,url:String) -> AmxResult<Cell>{

		let hashmap_guard = self.response_cache.clone();
		let response_context_guard = self.response_context_id.clone();
		let amx_guard:Arc<Mutex<AMX>> = Arc::new(Mutex::new(AMX::from(*amx.clone())));
		let amx_guard = amx_guard.clone();
		thread::spawn(move || {
			match Request::new(&url){
				Ok(mut http) =>{
					match http.get().send(){
						Ok(res) => {
							let body = res.text();
							hashmap_guard.lock().unwrap().insert(*response_context_guard.lock().unwrap(),body);
							*response_context_guard.lock().unwrap() += 1;
						}
						Err(err) =>{
							log!("Http error {:?}",err);
						}
					}
				}
				Err(err) =>{
					log!("Url parse error {:?}",err);
				}
			}
			
			let responseid = *response_context_guard.lock().unwrap() as Cell -1;
			let amx_unbox = amx_guard.lock().unwrap();
			exec_public!(amx_unbox, &callback;response_context_guard);
			
		});
		
		Ok(1)
	}

	fn delete_response_cache(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>{
		if self.response_cache.lock().unwrap().remove(&id) == None{
			log!("Error trying to remove invalid response id {:?}",id);
			Ok(0)
		}else{
			log!("[DEBUG] Removed response_data {:?}",id);
			Ok(1)
		}
	}

	fn delete_html_instance(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>{
		if self.html_instance.remove(&id) == None{
			log!("Error trying to remove invalid html id {:?}",id);
			Ok(0)
		}else{
			log!("[DEBUG] Removed html_instance {:?}",id);
			Ok(1)
		}
	}

	fn delete_selector_instance(&mut self,_:&AMX,id:usize) -> AmxResult<Cell>{
		if self.selectors.remove(&id) == None{
			log!("Error trying to remove invalid selector id {:?}",id);
			Ok(0)
		}else{
			log!("[DEBUG] Removed selector_instance {:?}",id);
			Ok(1)
		}
	}	
}


