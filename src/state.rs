// TODO Unfuckify this state struct, put it in the base app and
// partition each panel's substate out cause this is gonna get
// real VB.NETitive real quick

use std::fmt::Display;

pub struct AppState {
    pub focused_panel: Panel,

    pub current_method: Method,
    pub show_method_dropdown: bool,

    pub url_input: String,

    pub current_request_tab: RequestTab,
    pub request_headers: Vec<(String, String)>,
    pub request_body: String,
    pub request_settings: (), // TODO "fill in the type" i CANT i EATED it all
    pub focused_element: FocusedRequestHeadersElement,

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FocusedRequestHeadersElement {
    Tabs,
    Header(Option<u8>),
    AddButton,
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
