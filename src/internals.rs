use log::error;
use minihttp::request::Request;
use std::sync::mpsc::channel;

pub fn listen_for_http_calls(plugin: &mut super::PawnScraper) {
    let (http_request_complete_sender, http_request_complete_receiver) = channel();
    let (http_request_start_sender, http_request_start_receiver) = channel();

    plugin.http_request_complete_receiver = Some(http_request_complete_receiver);
    plugin.http_request_start_sender = Some(http_request_start_sender);

    std::thread::spawn(move || {
        for (playerid, callback, url, header) in http_request_start_receiver.iter() {
            match Request::new(&url) {
                Ok(mut http) => {
                    let method = if let Some(header) = header {
                        http.headers(header).get()
                    } else {
                        http.get()
                    };

                    match method.send() {
                        Ok(res) => {
                            let body = res.text();
                            http_request_complete_sender
                                .send((playerid, callback, body, true))
                                .unwrap();
                        }
                        Err(err) => {
                            error!("Http error {:?}", err);
                            http_request_complete_sender
                                .send((playerid, callback, String::from(""), false))
                                .unwrap();
                            //log!("Http error {:?} for url {:?}",err,url);
                        }
                    }
                }
                Err(_err) => {
                    http_request_complete_sender
                        .send((playerid, callback, String::from(""), false))
                        .unwrap();
                    //log!("Url parse error {:?} url is {:?}",err,url);
                }
            }
        }
    });
}
