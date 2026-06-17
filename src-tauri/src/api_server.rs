use std::sync::{Mutex, OnceLock, mpsc::{Sender, channel}};
use std::time::Duration;
use tiny_http::{Server, Response, Header, Method};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CapturedTab {
    #[serde(rename = "windowId")]
    pub window_id: i32,
    pub index: i32,
    pub url: String,
    pub title: String,
    pub active: bool,
}


struct ServerState {
    pending_polls: Vec<tiny_http::Request>,
    active_capture_sender: Option<Sender<String>>,
}

static STATE: OnceLock<Mutex<ServerState>> = OnceLock::new();

fn get_state() -> &'static Mutex<ServerState> {
    STATE.get_or_init(|| Mutex::new(ServerState {
        pending_polls: Vec::new(),
        active_capture_sender: None,
    }))
}

pub fn start_server() {
    std::thread::spawn(|| {
        let server = match Server::http("127.0.0.1:18942") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to start API server on port 18942: {}", e);
                return;
            }
        };

        for request in server.incoming_requests() {
            handle_request(request);
        }
    });
}

fn add_cors_headers<R: std::io::Read>(resp: Response<R>) -> Response<R> {
    resp.with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
        .with_header(Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS"[..]).unwrap())
        .with_header(Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap())
}

fn handle_request(mut request: tiny_http::Request) {
    if request.method() == &Method::Options {
        let resp = Response::empty(200);
        let resp = add_cors_headers(resp);
        let _ = request.respond(resp);
        return;
    }

    let url = request.url().to_string();

    if url == "/wait-for-capture" && request.method() == &Method::Get {
        let mut state = get_state().lock().unwrap();
        state.pending_polls.push(request);
        return;
    }

    if url == "/submit-tabs" && request.method() == &Method::Post {
        let mut content = String::new();
        if let Ok(_) = request.as_reader().read_to_string(&mut content) {
            let state = get_state().lock().unwrap();
            if let Some(ref sender) = state.active_capture_sender {
                let _ = sender.send(content);
            }
        }
        let resp = Response::from_string("{\"status\":\"ok\"}");
        let resp = add_cors_headers(resp);
        let _ = request.respond(resp);
        return;
    }

    let resp = Response::from_string("Not Found").with_status_code(404);
    let resp = add_cors_headers(resp);
    let _ = request.respond(resp);
}

pub fn trigger_capture_tabs() -> Option<String> {
    let (tx, rx) = channel();

    let mut pending_requests = Vec::new();
    {
        let mut state = get_state().lock().unwrap();
        state.active_capture_sender = Some(tx);
        std::mem::swap(&mut state.pending_polls, &mut pending_requests);
    }

    if pending_requests.is_empty() {
        std::thread::sleep(Duration::from_millis(100));
        {
            let mut state = get_state().lock().unwrap();
            std::mem::swap(&mut state.pending_polls, &mut pending_requests);
        }
    }

    for req in pending_requests {
        let resp = Response::from_string("{\"capture\":true}");
        let resp = add_cors_headers(resp);
        let _ = req.respond(resp);
    }

    let result = match rx.recv_timeout(Duration::from_millis(250)) {
        Ok(data) => Some(data),
        Err(_) => None,
    };

    {
        let mut state = get_state().lock().unwrap();
        state.active_capture_sender = None;
    }

    result
}
