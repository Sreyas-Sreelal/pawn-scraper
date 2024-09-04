use crate::internals::listen_for_http_calls;
use log::{error, info, warn};
use samp::amx::AmxIdent;
use samp::exec_public;
use samp::prelude::*;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

pub type HttpRequestSender =
    Option<Sender<(usize, String, String, Option<HashMap<String, String>>)>>;
pub type HttpRequestReceiver = Option<Receiver<(usize, String, String, bool)>>;

pub struct PawnScraper {
    pub plugin_version: i32,
    pub html_instance: HashMap<usize, Html>,
    pub selectors: HashMap<usize, Selector>,
    pub response_cache: HashMap<usize, String>,
    pub header_instance: HashMap<usize, HashMap<String, String>>,
    pub html_context_id: usize,
    pub selector_context_id: usize,
    pub response_context_id: usize,
    pub header_context_id: usize,
    pub http_request_start_sender: HttpRequestSender,
    pub http_request_complete_receiver: HttpRequestReceiver,
    pub amx_list: Vec<AmxIdent>,
}

impl SampPlugin for PawnScraper {
    fn on_load(&mut self) {
        listen_for_http_calls(self);

        info!(
            "
   ###############################################################
   #                      PawnScraper                            #
   #                      V0.2.2 Loaded!!                        #
   #   Found any bugs? Report it here:                           #
   #       https://github.com/Sreyas-Sreelal/pawn-scraper/issues #
   #                                                             #
   ###############################################################
			"
        );
    }

    fn on_unload(self: &mut PawnScraper) {
        info!("PawnScraper V0.2.2 Unloaded!!");
    }

    fn on_amx_load(&mut self, amx: &Amx) {
        self.amx_list.push(amx.ident());
        let get_version = amx.find_pubvar::<i32>("_pawnscraper_version");

        match get_version {
            Ok(version) => {
                if *version != self.plugin_version {
                    warn!("Warning plugin and include version doesnot match : Include {:?} Plugin {:?}",*version,self.plugin_version);
                }
            }
            Err(err) => {
                error!("Failed to retrive include version Reasone:{:?}\n You might want to update include ", err)
            }
        }
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        let raw = amx.ident();
        let index = self.amx_list.iter().position(|x| *x == raw).unwrap();
        self.amx_list.remove(index);
    }

    fn process_tick(&mut self) {
        for (playerid, callback, body, success) in self
            .http_request_complete_receiver
            .as_ref()
            .unwrap()
            .try_iter()
        {
            let body = body.as_str();
            let mut executed: bool = false;
            for amx in &self.amx_list {
                let mut responseid = -1;

                if success {
                    self.response_cache
                        .insert(self.response_context_id, String::from(body));
                    self.response_context_id += 1;
                    responseid = self.response_context_id as i32 - 1;
                }

                if let Some(amx) = samp::amx::get(*amx) {
                    let _ = exec_public!(amx, &callback, playerid, responseid);
                    executed = true;
                }
            }
            if !executed {
                error!("*Cannot execute callback {:?}", callback);
            }
        }
    }
}
