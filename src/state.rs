// TODO Unfuckify this state struct, put it in the base app and
// partition each panel's substate out cause this is gonna get
// real VB.NETitive real quick

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodState {
    pub current_method: Method,
    pub show_dropdown: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UrlState {
    pub url_input: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestState {
    pub headers: Vec<(String, String)>,
    pub current_header: RequestHeaderFocus,
    pub current_header_section: HeaderSection,
    pub body: String,
    pub settings: (),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseState {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestTab {
    Headers,
    Body,
    Settings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestHeaderFocus {
    Header(usize),
    Add,
}

impl PartialEq<usize> for RequestHeaderFocus {
    fn eq(&self, other: &usize) -> bool {
        match self {
            RequestHeaderFocus::Header(header_num) => header_num == other,
            RequestHeaderFocus::Add => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResponseTab {
    Data,
    Body,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HeaderSection {
    Name,
    Value,
    Delete,
}

impl HeaderSection {
    pub fn increment(&mut self) {
        *self = match self {
            Self::Name => Self::Value,
            Self::Value => Self::Delete,
            Self::Delete => Self::Delete,
        }
    }

    pub fn decrement(&mut self) {
        *self = match self {
            Self::Name => Self::Name,
            Self::Value => Self::Name,
            Self::Delete => Self::Value,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Name => Self::Value,
            Self::Value => Self::Delete,
            Self::Delete => Self::Delete,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Name => Self::Name,
            Self::Value => Self::Name,
            Self::Delete => Self::Value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Panel {
    Method,
    Url,
    Request(RequestTab),
    Response(ResponseTab),
}

impl Panel {
    pub fn increment(&mut self) {
        *self = match self {
            Self::Method => Self::Url,
            Self::Url => Self::Request(RequestTab::Headers),
            Self::Request(RequestTab::Headers) => Self::Request(RequestTab::Body),
            Self::Request(RequestTab::Body) => Self::Request(RequestTab::Settings),
            Self::Request(RequestTab::Settings) => Self::Response(ResponseTab::Data),
            Self::Response(ResponseTab::Data) => Self::Response(ResponseTab::Body),
            Self::Response(ResponseTab::Body) => Self::Method,
        }
    }

    pub fn decrement(&mut self) {
        *self = match self {
            Self::Method => Self::Response(ResponseTab::Body),
            Self::Url => Self::Method,
            Self::Request(RequestTab::Headers) => Self::Url,
            Self::Request(RequestTab::Body) => Self::Request(RequestTab::Headers),
            Self::Request(RequestTab::Settings) => Self::Request(RequestTab::Body),
            Self::Response(ResponseTab::Data) => Self::Request(RequestTab::Settings),
            Self::Response(ResponseTab::Body) => Self::Response(ResponseTab::Data),
        }
    }
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

impl Method {
    pub fn increment(&mut self) {
        *self = match self {
            Self::Get => Self::Post,
            Self::Post => Self::Put,
            Self::Put => Self::Patch,
            Self::Patch => Self::Options,
            Self::Options => Self::Connect,
            Self::Connect => Self::Trace,
            Self::Trace => Self::Delete,
            Self::Delete => Self::Head,
            Self::Head => Self::Get,
        }
    }

    pub fn decrement(&mut self) {
        *self = match self {
            Self::Get => Self::Head,
            Self::Post => Self::Get,
            Self::Put => Self::Post,
            Self::Patch => Self::Put,
            Self::Options => Self::Patch,
            Self::Connect => Self::Options,
            Self::Trace => Self::Connect,
            Self::Delete => Self::Trace,
            Self::Head => Self::Delete,
        }
    }
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
