use log::{info, warn, error};
use proxy_wasm::{traits::*, types::*};
use serde::Deserialize;
use serde_json_wasm::de;
use std::time::Duration;

// -----------------------------------------------------------------------------
// Config
// -----------------------------------------------------------------------------

#[derive(Deserialize, Clone, Copy, Debug)]
struct Config {
    #[serde(default = "default_600")]
    tick_period: u64,

    #[serde(default = "default_negative_1")]
    required_parameter: i32,
}

fn default_600() -> u64 {
    600
}

fn default_negative_1() -> i32 {
    -1
}

// -----------------------------------------------------------------------------
// Root Context
// -----------------------------------------------------------------------------

const ROOT_ID: u32 = 0;

struct MyFilterRoot {
    config: Option<Config>,
}

struct MyFilter {
    context_id: u32,
    _config: Config,
}

impl Context for MyFilterRoot {
    fn on_http_call_response(
        &mut self,
        token_id: u32,
        num_headers: usize,
        body_size: usize,
        _num_trailers: usize,
    ) {
        info!("#{} on_http_call_response [root], token_id: {}, num_headers: {}, body_size: {}",
            ROOT_ID, token_id, num_headers, body_size
        );
    }

    fn on_done(&mut self) -> bool {
        info!("#{} on_done", ROOT_ID);
        true
    }
}

impl RootContext for MyFilterRoot {
    fn on_vm_start(&mut self, config_size: usize) -> bool {
        info!("#{} on_vm_start, config_size: {}", ROOT_ID, config_size);
        true
    }

    fn on_configure(&mut self, config_size: usize) -> bool {
        info!("#{} on_configure, config_size: {}", ROOT_ID, config_size);

        if let Some(config_bytes) = self.get_plugin_configuration() {
            assert!(config_bytes.len() == config_size);
            match de::from_slice::<Config>(&config_bytes) {
                Ok(config) => {
                    if config.required_parameter == -1 {
                        error!(
                            "#{} on_configure: required_parameter not found {:?}",
                            ROOT_ID, self.config.unwrap()
                        );
                    }

                    self.config = Some(config);
                    self.set_tick_period(Duration::from_secs(config.tick_period));

                    info!(
                        "#{} on_configure: loaded configuration: {:?}",
                        ROOT_ID, self.config.unwrap()
                    );

                    true
                }
                Err(err) => {
                    warn!(
                        "#{} on_configure: failed parsing configuration: {}: {}",
                        ROOT_ID, String::from_utf8(config_bytes).unwrap(), err
                    );

                    false
                }
            }
        } else {
            warn!("#{} on_configure: failed getting configuration", ROOT_ID);

            false
        }
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    // Called when the host environment creates a new HTTP context
    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        info!("#{} create_http_context: context_id: {}", ROOT_ID, context_id);

        if let Some(config) = &self.config {
            Some(Box::new(MyFilter {
                context_id: context_id,
                _config: config.clone(),
            }))
        } else {
            None
        }
    }

    fn on_tick(&mut self) {
        info!("#{} on_tick", ROOT_ID);

        self.dispatch_http_call(
            "mockbin.org:80",
            vec![
                (":path", "/request/foo"),
                (":method", "GET"),
                (":scheme", "http"),
                (":authority", "mockbin.org:80"),
            ],
            None,
            vec![],
            Duration::from_secs(4),
        )
        .unwrap();
    }
}

// -----------------------------------------------------------------------------
// Plugin Context
// -----------------------------------------------------------------------------

impl Context for MyFilter {
    fn on_http_call_response(
        &mut self,
        token_id: u32,
        nheaders: usize,
        body_size: usize,
        _num_trailers: usize,
    ) {
        info!(
            "#{} on_http_call_response, token_id: {}, num_headers: {}, body_size: {}",
            self.context_id, token_id, nheaders, body_size
        );
    }
}

impl HttpContext for MyFilter {
    fn on_http_request_headers(&mut self, nheaders: usize, _eof: bool) -> Action {
        info!("#{} on_request_headers, headers: {}", self.context_id, nheaders);

        Action::Pause
    }

    fn on_http_request_body(&mut self, body_size: usize, eof: bool) -> Action {
        info!(
            "#{} on_request_body, body_size: {}, eof: {}",
            self.context_id, body_size, eof
        );

        Action::Continue
    }

    fn on_http_response_headers(&mut self, nheaders: usize, _eof: bool) -> Action {
        info!(
            "#{} on_response_headers, headers: {}",
            self.context_id, nheaders
        );

        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, eof: bool) -> Action {
        info!(
            "#{} on_response_body, body_size: {}, eof: {}",
            self.context_id, body_size, eof
        );

        Action::Continue
    }

    fn on_log(&mut self) {
        info!("#{} on_log", self.context_id);

        self.dispatch_http_call(
            "127.0.0.1:9000",
            vec![
                (":method", "GET"),
                (":path", "/foo"),
                (":authority", "127.0.0.1:9000"),
            ],
            None,
            vec![],
            Duration::from_secs(5),
        )
        .unwrap();
    }
}

proxy_wasm::main! {{

    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MyFilterRoot {
            config: None,
        })
    });
}}
