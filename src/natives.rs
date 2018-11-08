use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};
use scraper::{Html,Selector};

pub trait Natives {
	
	fn parse_document(&mut self,_:&AMX,document:String) -> AmxResult<Cell>;
	fn parse_selector(&mut self,_:&AMX,string:String) -> AmxResult<Cell>;
}

impl<'a,'b> Natives for super::PawnScraper<'a,'b>{
	
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
		
}


