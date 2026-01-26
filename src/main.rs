mod errors;
mod keyboard;
mod settings;
mod state;
mod ui;

use chrono::prelude::Utc;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, EnableBracketedPaste, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures_util::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};
use reqwest::{
    Client, Response, StatusCode, Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use ropey::Rope;
use std::{
    io,
    sync::{Arc, Mutex},
};
use unicode_bom::Bom;

use crate::{
    errors::SendRequestError,
    settings::Settings,
    state::{
        BodyCursor, HeaderSection, MainState, Method, MethodState, Panel, RequestHeaderFocus,
        UrlState,
    },
    ui::animations::AnimationState,
};

pub enum ResponseType {
    None,
    Pending,
    FinishedSuccess(WrappedResponse),
    FinishedError(SendRequestError),
}

impl ResponseType {
    pub fn append_body(&mut self, chunk: String) -> bool {
        if let Self::FinishedSuccess(response) = self {
            response.append_body(chunk);
            true
        } else {
            false
        }
    }

    pub fn set_body_status(&mut self, status: BodyStatus) -> bool {
        if let Self::FinishedSuccess(response) = self {
            response.set_body_status(status);
            true
        } else {
            false
        }
    }
}

impl AsRef<Self> for ResponseType {
    fn as_ref(&self) -> &Self {
        self
    }
}
pub struct WrappedResponse {
    meta: WrappedResponseMeta,
    body: String,
    body_status: BodyStatus,
}

impl WrappedResponse {
    pub fn append_body(&mut self, chunk: String) {
        self.body.push_str(chunk.as_str());
    }

    pub fn set_body_status(&mut self, status: BodyStatus) {
        self.body_status = status;
    }
}

pub struct WrappedResponseMeta {
    url: Url,
    status: StatusCode,
    headers: HeaderMap,
}

pub enum BodyStatus {
    Streaming,
    Finished,
}

impl BodyStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Streaming => "...",
            Self::Finished => " ✓",
        }
    }
}

pub struct Application {
    pub settings: Settings,
    pub focused_panel: Panel,
    pub method_state: MethodState,
    pub url_state: UrlState,
    pub main_state: MainState,
    pub editing: bool,
    pub exit_request: bool,

    pub client: Client,

    pub last_response: Arc<Mutex<ResponseType>>,

    pub animation_state: AnimationState,
}

impl Application {
    fn new() -> Self {
        Self {
            settings: Settings::load_from_config(),

            focused_panel: Panel::Method,
            method_state: MethodState {
                current_method: Method::Get,
                show_dropdown: false,
            },
            url_state: UrlState {
                url_input: String::new(),
                url_cursor: 0,
            },
            main_state: MainState {
                headers: Vec::new(),
                current_header: RequestHeaderFocus::Add,
                current_header_section: HeaderSection::Name,
                current_header_cursor: 0,
                request_body: Rope::new(),
                request_body_cursor: BodyCursor {
                    position: 0,
                    target_character: 0,
                },
            },
            editing: false,
            exit_request: false,

            client: Client::new(),

            last_response: Arc::new(Mutex::new(ResponseType::None)),

            animation_state: Default::default(),
        }
    }

    fn run<T: ratatui::backend::Backend + std::io::Write>(
        &mut self,
        terminal: &mut Terminal<T>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                    execute!(
                        terminal.backend_mut(),
                        if self.editing {
                            SetCursorStyle::BlinkingBar
                        } else {
                            SetCursorStyle::SteadyBlock
                        }
                    )?;
                }
            }

            if self.exit_request {
                return Ok(());
            }
        }
    }

    pub fn send_request(&mut self) {
        let client = self.client.clone();
        let method = self.method_state.current_method.clone();
        let url = self.url_state.url_input.clone();
        let headers = self.main_state.headers.clone();
        let body = self.main_state.request_body.clone();

        let response_mtx = self.last_response.clone();

        tokio::spawn(async move {
            *response_mtx.lock().unwrap() = ResponseType::Pending;
            let result = send_request_async(client, method, url, headers, body).await;

            match result {
                Ok(response) => {
                    let wrapped = WrappedResponse {
                        meta: WrappedResponseMeta {
                            url: response.url().clone(),
                            status: response.status(),
                            headers: response.headers().clone(),
                        },
                        body: "".to_string(),
                        body_status: BodyStatus::Streaming,
                    };

                    *response_mtx.lock().unwrap() = ResponseType::FinishedSuccess(wrapped);

                    let mut body_stream = response.bytes_stream();

                    while let Some(chunk) = body_stream.next().await {
                        if let Ok(chunk) = chunk {
                            let chunk_str = decode_with_unicode_bom(chunk.iter().as_slice());
                            response_mtx.lock().unwrap().append_body(chunk_str);
                        } else {
                            break; // napoleon meme
                        }
                    }

                    response_mtx
                        .lock()
                        .unwrap()
                        .set_body_status(BodyStatus::Finished);
                }
                Err(e) => {
                    *response_mtx.lock().unwrap() = ResponseType::FinishedError(e);
                }
            }
        });
    }
}

fn decode_with_unicode_bom(bytes: &[u8]) -> String {
    let bom = Bom::from(bytes);

    match bom {
        Bom::Utf8 => String::from_utf8_lossy(&bytes[bom.len()..]).into_owned(),
        Bom::Utf16Le => {
            let utf16: Vec<u16> = bytes[bom.len()..]
                .chunks(2)
                .map(|c| u16::from_le_bytes([c[0], c.get(1).copied().unwrap_or(0)]))
                .collect();
            String::from_utf16_lossy(&utf16)
        }
        Bom::Utf16Be => {
            let utf16: Vec<u16> = bytes[bom.len()..]
                .chunks(2)
                .map(|c| u16::from_be_bytes([c[0], c.get(1).copied().unwrap_or(0)]))
                .collect();
            String::from_utf16_lossy(&utf16)
        }
        _ => String::from_utf8_lossy(bytes).into_owned(),
    }
}

async fn send_request_async(
    client: Client,
    method: Method,
    url: String,
    headers: Vec<(String, String)>,
    body: Rope,
) -> Result<Response, SendRequestError> {
    let mut header_map: HeaderMap = HeaderMap::default();

    for (n, v) in headers.iter() {
        let name = HeaderName::from_bytes(n.as_bytes()).map_err(|error| {
            SendRequestError::InvalidHeaderName {
                name: n.clone(),
                error,
            }
        })?;
        let value = HeaderValue::from_bytes(v.as_bytes()).map_err(|error| {
            SendRequestError::InvalidHeaderValue {
                value: v.clone(),
                error,
            }
        })?;

        header_map.insert(name, value);
    }

    let mut request = client.request(method.into(), url);

    let body = body.to_string();
    if !body.trim().is_empty() {
        // try to serialize json body for validation purposes, then reserialize as minified
        let body_desered = serde_json::from_str::<serde_json::Value>(&body)
            .map_err(SendRequestError::InvalidBody)?;

        // I pray I don't get cloudflared                                here
        let body_minified = serde_json::to_string(&body_desered).unwrap();

        request = request.body(body_minified);
    }

    let result = request.send().await?;

    Ok(result)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let log_folder = format!(
        "{}/logs/",
        std::env::var("$HOME").unwrap_or(".".to_string())
    );

    std::fs::create_dir_all(&log_folder)
        .unwrap_or_else(|_| panic!("Could not create log folder in {}", log_folder));

    fern::Dispatch::new()
        .chain(
            fern::log_file(format!(
                "./logs/{}.log",
                Utc::now().naive_utc().format("%Y%m%d_%H%M%S")
            ))
            .unwrap(),
        )
        .apply()
        .unwrap();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, EnableBracketedPaste)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = Application::new().run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}
