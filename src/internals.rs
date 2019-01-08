use std::sync::mpsc::channel;
use minihttp::request::Request;

pub trait Internal {
	fn listen_for_http_calls(&mut self);
}

impl Internal for super::PawnScraper {
    fn listen_for_http_calls(&mut self){
        let (http_request_complete_sender, http_request_complete_receiver) = channel();
		let (http_request_start_sender, http_request_start_receiver) = channel();
		
		self.http_request_complete_receiver = Some(http_request_complete_receiver);
		self.http_request_start_sender = Some(http_request_start_sender);
			
		std::thread::spawn(move || {
			for (playerid,callback,url) in http_request_start_receiver.iter() {
				match Request::new(&url){
					Ok(mut http) =>{
						match http.get().send(){
							Ok(res) => {
								let body = res.text();
								http_request_complete_sender.send((playerid, callback, body,true)).unwrap();
							}
							Err(_err) =>{
								http_request_complete_sender.send((playerid, callback, String::from(""),false)).unwrap();
								//log!("Http error {:?} for url {:?}",err,url);
							}
						}
					}
					Err(_err) =>{
						http_request_complete_sender.send((playerid, callback, String::from(""),false)).unwrap();
						//log!("Url parse error {:?} url is {:?}",err,url);
					}
				}
			}	
		});
    }

}