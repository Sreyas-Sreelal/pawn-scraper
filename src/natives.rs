use crate::encode::encode_replace;
use log::{debug, error, warn};
use minihttp::request::Request;
use samp::args::Args;
use samp::native;
use samp::prelude::*;
use scraper::{Html, Selector};
use std::collections::HashMap;

impl super::PawnScraper {
    #[native(name = "ParseHtmlDocument")]
    pub fn parse_document(&mut self, _: &Amx, document: AmxString) -> AmxResult<i32> {
        let parsed_data = Html::parse_document(&document.to_string());
        self.html_instance.insert(self.html_context_id, parsed_data);
        self.html_context_id += 1;
        Ok(self.html_context_id as i32 - 1)
    }

    #[native(name = "ResponseParseHtml")]
    pub fn parse_document_by_response(&mut self, _: &Amx, id: usize) -> AmxResult<i32> {
        if id > self.response_context_id {
            Ok(-1)
        } else {
            let response_data = self.response_cache.get(&id);
            if response_data == None {
                Ok(-1)
            } else {
                let parsed_data = Html::parse_document(&response_data.unwrap());
                self.html_instance.insert(self.html_context_id, parsed_data);
                self.html_context_id += 1;
                Ok(self.html_context_id as i32 - 1)
            }
        }
    }

    #[native(name = "ParseSelector")]
    pub fn parse_selector(&mut self, _: &Amx, string: AmxString) -> AmxResult<i32> {
        match Selector::parse(&string.to_string()) {
            Ok(selector) => {
                self.selectors.insert(self.selector_context_id, selector);
                self.selector_context_id += 1;
                Ok(self.selector_context_id as i32 - 1)
            }
            Err(err) => {
                error!("Failed parsing selector {:?}", err);
                Ok(-1)
            }
        }
    }

    #[native(name = "GetNthElementText")]
    pub fn get_nth_element_text(
        &mut self,
        _: &Amx,
        docid: usize,
        selectorid: usize,
        idx: usize,
        string: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.html_instance.contains_key(&docid) || !self.selectors.contains_key(&selectorid) {
            debug!(
                "Invalid html instances passed docid {:?},selectorid {:?}",
                docid, selectorid
            );
            Ok(-1)
        } else {
            let html = &self.html_instance.get(&docid).unwrap();
            let selector = &self.selectors.get(&selectorid).unwrap();
            let nth_element = html.select(selector).nth(idx);
            if nth_element == None {
                Ok(0)
            } else {
                let element_text_iter = nth_element.unwrap().text();
                let mut full_text: String = String::new();
                for i in element_text_iter {
                    full_text += i;
                }
                match encode_replace(&full_text) {
                    Ok(text_encoded) => {
                        let mut dest = string.into_sized_buffer(size);
                        let _ = samp::cell::string::put_in_buffer(&mut dest, &text_encoded);
                        Ok(1)
                    }
                    Err(err) => {
                        error!("Encoding error {:?}", err);
                        Ok(0)
                    }
                }
            }
        }
    }

    #[native(name = "GetNthElementName")]
    pub fn get_nth_element_name(
        &mut self,
        _: &Amx,
        docid: usize,
        selectorid: usize,
        idx: usize,
        string: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.html_instance.contains_key(&docid) || !self.selectors.contains_key(&selectorid) {
            error!(
                "Invalid html instances passed docid {:?},selectorid {:?}",
                docid, selectorid
            );
            Ok(-1)
        } else {
            let html = &self.html_instance.get(&docid).unwrap();
            let selector = &self.selectors.get(&selectorid).unwrap();
            let nth_element = html.select(selector).nth(idx);

            if nth_element == None {
                Ok(0)
            } else {
                let element_name = nth_element.unwrap().value().name();
                match encode_replace(element_name) {
                    Ok(name_encoded) => {
                        let mut dest = string.into_sized_buffer(size);
                        let _ = samp::cell::string::put_in_buffer(&mut dest, &name_encoded);
                    }
                    Err(err) => {
                        error!("Encoding error {:?}", err);
                    }
                }

                Ok(1)
            }
        }
    }

    #[native(name = "GetNthElementAttrVal")]
    pub fn get_nth_element_attr_value(
        &mut self,
        _: &Amx,
        docid: usize,
        selectorid: usize,
        idx: usize,
        attr: AmxString,
        string: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.html_instance.contains_key(&docid) || !self.selectors.contains_key(&selectorid) {
            error!(
                "Invalid html instances passed docid {:?},selectorid {:?}",
                docid, selectorid
            );
            Ok(-1)
        } else {
            let html = &self.html_instance.get(&docid).unwrap();
            let selector = &self.selectors.get(&selectorid).unwrap();
            let nth_element = html.select(selector).nth(idx);
            if nth_element == None {
                Ok(0)
            } else {
                let attr_value = nth_element.unwrap().value().attr(&attr.to_string());
                if attr_value == None {
                    Ok(-2)
                } else {
                    match encode_replace(attr_value.unwrap()) {
                        Ok(attr_encoded) => {
                            let mut dest = string.into_sized_buffer(size);
                            let _ = samp::cell::string::put_in_buffer(&mut dest, &attr_encoded);
                            Ok(1)
                        }
                        Err(err) => {
                            error!("Encoding error {:?}", err);
                            Ok(0)
                        }
                    }
                }
            }
        }
    }

    #[native(name = "HttpGet")]
    pub fn http_request(&mut self, _: &Amx, url: AmxString, headerid: usize) -> AmxResult<i32> {
        let header: Option<HashMap<String, String>>;

        if !self.header_instance.contains_key(&headerid) {
            header = None;
        } else {
            header = Some(self.header_instance.get(&headerid).unwrap().clone());
        }
        match Request::new(&url.to_string()) {
            Ok(mut http) => {
                let method;

                if header == None {
                    method = http.get();
                } else {
                    method = http.headers(header.unwrap()).get();
                }

                match method.send() {
                    Ok(res) => {
                        let body = res.text();

                        self.response_cache.insert(self.response_context_id, body);
                        self.response_context_id += 1;

                        Ok(self.response_context_id as i32 - 1)
                    }
                    Err(err) => {
                        error!("Http error {:?}", err);
                        Ok(-1)
                    }
                }
            }
            Err(err) => {
                error!("Url parse error {:?}", err);
                Ok(-1)
            }
        }
    }

    #[native(name = "HttpGetThreaded")]
    pub fn http_request_threaded(
        &mut self,
        _: &Amx,
        playerid: usize,
        callback: AmxString,
        url: AmxString,
        headerid: usize,
    ) -> AmxResult<i32> {
        let header: Option<HashMap<String, String>>;

        if !self.header_instance.contains_key(&headerid) {
            header = None;
        } else {
            header = Some(self.header_instance.get(&headerid).unwrap().clone());
        }

        self.http_request_start_sender
            .as_ref()
            .unwrap()
            .send((playerid, callback.to_string(), url.to_string(), header))
            .unwrap();
        Ok(1)
    }

    #[native(name = "DeleteResponse")]
    pub fn delete_response_cache(&mut self, _: &Amx, id: usize) -> AmxResult<i32> {
        if self.response_cache.remove(&id) == None {
            warn!("trying to remove invalid response id {:?}", id);
            Ok(0)
        } else {
            //info!("Removed response_data {:?}",id);
            Ok(1)
        }
    }

    #[native(name = "DeleteHtml")]
    pub fn delete_html_instance(&mut self, _: &Amx, id: usize) -> AmxResult<i32> {
        if self.html_instance.remove(&id) == None {
            warn!("Warning trying to remove invalid html id {:?}", id);
            Ok(0)
        } else {
            //info!("Removed html_instance {:?}",id);
            Ok(1)
        }
    }

    #[native(name = "DeleteSelector")]
    pub fn delete_selector_instance(&mut self, _: &Amx, id: usize) -> AmxResult<i32> {
        if self.selectors.remove(&id) == None {
            warn!("Warning trying to remove invalid selector id {:?}", id);
            Ok(0)
        } else {
            //info!("Removed selector_instance {:?}",id);
            Ok(1)
        }
    }

    #[native(name = "DeleteHeader")]
    pub fn delete_header_instance(&mut self, _: &Amx, id: usize) -> AmxResult<i32> {
        if self.header_instance.remove(&id) == None {
            warn!("Warning trying to remove invalid header object id {:?}", id);
            Ok(0)
        } else {
            //info!("Removed selector_instance {:?}",id);
            Ok(1)
        }
    }

    #[native(raw, name = "CreateHeader")]
    pub fn create_header(&mut self, _: &Amx, mut args: Args) -> AmxResult<i32> {
        let params_count = args.count();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut isok: bool = true;
        let mut key: Option<AmxString>;
        let mut value: Option<AmxString>;

        if params_count % 2 == 0 && params_count != 0 {
            for _ in (1..params_count).step_by(2) {
                key = args.next::<AmxString>();
                if key.is_none() {
                    isok = false;
                    break;
                }

                value = args.next::<AmxString>();
                if value.is_none() {
                    isok = false;
                    break;
                }

                headers.insert(key.unwrap().to_string(), value.unwrap().to_string());
            }

            if !isok {
                Ok(-1)
            } else {
                self.header_instance.insert(self.header_context_id, headers);
                self.header_context_id += 1;
                Ok(self.header_context_id as i32 - 1)
            }
        } else {
            error!("Error Invalid number of parameters passed to function CreateHeader");
            Ok(-1)
        }
    }
}
