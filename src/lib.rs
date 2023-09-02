#[macro_use]
mod macros;
mod encode;
mod internals;
mod natives;
mod plugin;

use crate::plugin::PawnScraper;
use samp::initialize_plugin;
use std::collections::HashMap;

initialize_plugin!(
    natives: [
        PawnScraper::parse_document,
        PawnScraper::parse_document_by_response,
        PawnScraper::parse_selector,
        PawnScraper::http_request,
        PawnScraper::http_request_threaded,
        PawnScraper::get_nth_element_name,
        PawnScraper::get_nth_element_text,
        PawnScraper::get_nth_element_attr_value,
        PawnScraper::delete_html_instance,
        PawnScraper::delete_selector_instance,
        PawnScraper::delete_response_cache,
        PawnScraper::delete_header_instance,
        PawnScraper::create_header
    ],
    {
        samp::plugin::enable_process_tick();

        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let log_file = fern::log_file("PawnScraper.log").expect("Cannot create log file!");

        let trace_level = fern::Dispatch::new()
            .level(log::LevelFilter::Info)
            .chain(log_file);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[PawnScraper] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .chain(trace_level)
            .apply();

        PawnScraper {
            plugin_version: 21,
            html_instance: HashMap::new(),
            selectors: HashMap::new(),
            response_cache: HashMap::new(),
            header_instance: HashMap::new(),
            html_context_id: 0,
            selector_context_id: 0,
            response_context_id: 0,
            header_context_id: 0,
            http_request_start_sender: None,
            http_request_complete_receiver: None,
            amx_list: Vec::new(),
        }
    }
);
