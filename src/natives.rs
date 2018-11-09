use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};
use scraper::{Html,Selector};

pub trait Natives {
	fn parse_document(&mut self,_:&AMX,document:String) -> AmxResult<Cell>;
	fn parse_selector(&mut self,_:&AMX,string:String) -> AmxResult<Cell>;
	fn get_nth_element_name(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>;
	fn get_nth_element_text(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>;
}

impl Natives for super::PawnScraper{
	fn parse_document(&mut self,_:&AMX,document:String) -> AmxResult<Cell> {
		let parsed_data = Html::parse_document(&document);
		self.html_instance.push(parsed_data);
		Ok(self.html_instance.len()  as Cell -1)
	}

	fn parse_selector(&mut self,_:&AMX,string:String) -> AmxResult<Cell> {
		match Selector::parse(&string){
			Ok(selector) => {
				self.selectors.push(selector);
				Ok(self.selectors.len() as Cell -1)
			}
			Err(err) =>{
				log!("Failed parsing selector {:?}",err);
				Ok(-1)
			}
		}
	}
	fn get_nth_element_text(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>{
		if docid >= self.html_instance.len() || selectorid >= self.selectors.len(){
			log!("Invalid html instances passed docid {:?},selectorid {:?}",docid,selectorid);
			Ok(-1)
		}else{
			let html = &self.html_instance[docid];
			let selector = &self.selectors[selectorid];
			let nth_element = html.select(selector).nth(idx);
			if nth_element == None{
				log!("Error on fetching element {:?} idx {:?} docid {:?} selectorid {:?}",nth_element,idx,docid,selectorid);
				Ok(0)
			}else{
				let element_text_iter = nth_element.unwrap().text();
				let mut full_text:String = String::new();
				for i in element_text_iter{
					full_text += i;
				}
				log!("Testing element_text {:?}",full_text);
				let text_encoded = samp_sdk::cp1251::encode(&full_text).unwrap();
				set_string!(text_encoded,string,size);
				Ok(1)
			}
		}
	}
	fn get_nth_element_name(&mut self,_:&AMX,docid:usize, selectorid:usize,idx:usize,string:&mut Cell,size:usize) -> AmxResult<Cell>{
		if docid >= self.html_instance.len() || selectorid >= self.selectors.len(){
			log!("Invalid html instances passed docid {:?},selectorid {:?}",docid,selectorid);
			Ok(-1)
		}else{
			let html = &self.html_instance[docid];
			let selector = &self.selectors[selectorid];
			let nth_element = html.select(selector).nth(idx);
			
			if nth_element == None{
				log!("Error on fetching element {:?} idx {:?} docid {:?} selectorid {:?}",self.html_instance[docid].select(&self.selectors[selectorid]).nth(idx),idx,docid,selectorid);
				Ok(0)
			}else{
				let element_name = nth_element.unwrap().value().name();
				log!("Testing element_name {:?}",element_name);
				let name_encoded = samp_sdk::cp1251::encode(element_name).unwrap();
				set_string!(name_encoded,string,size);
				Ok(1)
			}
		}
	}
}


