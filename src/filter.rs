use log::{info, warn};
use proxy_wasm::{traits::*, types::*};
use serde::Deserialize;
use serde_json_wasm::de;

// -----------------------------------------------------------------------------
// Config
// -----------------------------------------------------------------------------

#[derive(Deserialize, Clone, Copy, Debug)]
struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    my_status_code: Option<u32>,
}

// -----------------------------------------------------------------------------
// Root Context
// -----------------------------------------------------------------------------

struct MyFilterRoot {
    config: Option<Config>,
}

struct MyFilter {
    context_id: u32,
    config: Config,
}

impl Context for MyFilterRoot {
//    fn on_http_call_response(
//        &mut self,
//        token_id: u32,
//        num_headers: usize,
//        body_size: usize,
//        _num_trailers: usize,
//    ) {
//    }
//
//    fn on_done(&mut self) -> bool {
//        true
//    }
}

impl RootContext for MyFilterRoot {
//    fn on_vm_start(&mut self, config_size: usize) -> bool {
//        true
//    }
//
//    fn on_tick(&mut self) {
//    }

    fn on_configure(&mut self, config_size: usize) -> bool {
        info!("on_configure, config_size: {}", config_size);

        if let Some(config_bytes) = self.get_plugin_configuration() {
            match de::from_slice::<Config>(&config_bytes) {
                Ok(config) => {
                    self.config = Some(config);

                    true
                }
                Err(err) => {
                    warn!(
                        "on_configure: failed parsing configuration: {}: {}",
                        String::from_utf8(config_bytes).unwrap(), err
                    );

                    false
                }
            }
        } else {
            warn!("on_configure: failed getting configuration");

            false
        }
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        info!("create_http_context: context_id: {}", context_id);

        if let Some(config) = &self.config {
            Some(Box::new(MyFilter {
                context_id: context_id,
                config: config.clone(),
            }))
        } else {
            None
        }
    }
}

// -----------------------------------------------------------------------------
// Plugin Context
// -----------------------------------------------------------------------------

impl Context for MyFilter {
//    fn on_http_call_response(
//        &mut self,
//        token_id: u32,
//        nheaders: usize,
//        body_size: usize,
//        _num_trailers: usize,
//    ) {}
}

impl HttpContext for MyFilter {
//    fn on_http_request_headers(&mut self, nheaders: usize, _eof: bool) -> Action {
//        Action::Continue
//    }
//
//    fn on_http_request_body(&mut self, body_size: usize, eof: bool) -> Action {
//        Action::Continue
//    }
//
    fn on_http_response_headers(&mut self, nheaders: usize, _eof: bool) -> Action {
        info!("#{} on_response_headers, headers: {}", self.context_id, nheaders);

        match self.config.my_status_code {
            Some(status) => {
                self.set_http_response_header("status", Some(&status.to_string()))
            },
            None => ()
        }

        Action::Continue
    }
//
//    fn on_http_response_body(&mut self, body_size: usize, eof: bool) -> Action {
//        Action::Continue
//    }
//
//    fn on_log(&mut self) {
//    }
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MyFilterRoot {
            config: None,
        })
    });
}}
