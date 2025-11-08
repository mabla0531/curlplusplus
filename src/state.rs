use std::fmt::Display;

pub struct AppState {
    pub focused_panel: Panel,

    pub current_method: Method,
    pub show_method_dropdown: bool,

    pub url_input: String,

    pub current_request_tab: RequestTab,
    pub current_response_tab: ResponseTab,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestTab {
    Headers,
    Body,
    Settings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResponseTab {
    Data,
    Body,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Panel {
    Method,
    Url,
    Request,
    Response,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Options,
    Connect,
    Trace,
    Delete,
    Head,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "Get"),
            Self::Post => write!(f, "Post"),
            Self::Put => write!(f, "Put"),
            Self::Patch => write!(f, "Patch"),
            Self::Options => write!(f, "Options"),
            Self::Connect => write!(f, "Connect"),
            Self::Trace => write!(f, "Trace"),
            Self::Delete => write!(f, "Delete"),
            Self::Head => write!(f, "Head"),
        }
    }
}
