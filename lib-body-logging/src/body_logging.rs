use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(BodyLogger { context_id })
    });
}

struct BodyLogger {
    context_id: u32,
}

impl Context for BodyLogger {}

impl HttpContext for BodyLogger {

    fn on_http_request_headers(&mut self, _: usize) -> Action {
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Request Headers").as_str());

        self.set_http_request_header("X-HV-REQUEST-HEADER", Some("SETSOMETHING"));

        for (name, value) in &self.get_http_request_headers() {
            proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Request {name}={value}", name=name, value=value).as_str());
        }

        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Request Payload").as_str());

        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered at the host side.
            return Action::Pause;
        }

        // Since we returned "Pause" previuously, this will return the whole body.
        if let Some(body_bytes) = self.get_http_request_body(0, body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();

            proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Request {}", body_str).as_str());

            if body_str.contains("WASM") {
                let new_body = format!("BODY MANIPULATED {}", body_str);
                self.set_http_request_body(0, new_body.len(), &new_body.into_bytes());
            }
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Response Headers").as_str());

        self.set_http_response_header("X-HV-RESPONSE-HEADER", Some("SETSOMETHING"));

        for (name, value) in &self.get_http_response_headers() {
            proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Response {name}={value}", name=name, value=value).as_str());
        }

        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Response Payload").as_str());

        if !end_of_stream {
            // Wait -- we'll be called again when the complete body is buffered at the host side.
            return Action::Pause;
        }

        // Since we returned "Pause" previuously, this will return the whole body.
        if let Some(body_bytes) = self.get_http_response_body(0, body_size) {
            let body_str = String::from_utf8(body_bytes).unwrap();

            proxy_wasm::hostcalls::log(LogLevel::Info, format!("BodyLogger : Response {}", body_str).as_str());

            if body_str.contains("WASM") {
                let new_body = format!("BODY MANIPULATED {}", body_str);
                self.set_http_response_body(0, new_body.len(), &new_body.into_bytes());
            }
        }
        Action::Continue

    }

}
