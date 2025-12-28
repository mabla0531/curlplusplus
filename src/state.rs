use ropey::Rope;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodState {
    pub current_method: Method,
    pub show_dropdown: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UrlState {
    pub url_input: String,
    pub url_cursor: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MainState {
    pub headers: Vec<(String, String)>,
    pub current_header: RequestHeaderFocus,
    pub current_header_section: HeaderSection,
    pub current_header_cursor: usize,
    pub request_body: Rope,
    pub request_body_cursor: BodyCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BodyCursor {
    pub position: usize,
    pub target_character: usize,
}

impl BodyCursor {
    pub fn as_line(&self, rope: &Rope) -> usize {
        rope.char_to_line(self.position)
    }

    pub fn as_line_start(&self, rope: &Rope) -> usize {
        rope.line_to_char(rope.char_to_line(self.position))
    }

    pub fn as_char_in_line(&self, rope: &Rope) -> usize {
        self.position - self.as_line_start(rope)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainTab {
    RequestHeaders,
    RequestBody,
    ResponseData,
    ResponseBody,
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
    Main(MainTab),
}

impl Panel {
    pub fn increment(&mut self) {
        *self = match self {
            Self::Method => Self::Url,
            Self::Url => Self::Main(MainTab::RequestHeaders),
            Self::Main(MainTab::RequestHeaders) => Self::Main(MainTab::RequestBody),
            Self::Main(MainTab::RequestBody) => Self::Main(MainTab::ResponseData),
            Self::Main(MainTab::ResponseData) => Self::Main(MainTab::ResponseBody),
            Self::Main(MainTab::ResponseBody) => Self::Method,
        }
    }

    pub fn decrement(&mut self) {
        *self = match self {
            Self::Method => Self::Main(MainTab::ResponseBody),
            Self::Url => Self::Method,
            Self::Main(MainTab::RequestHeaders) => Self::Url,
            Self::Main(MainTab::RequestBody) => Self::Main(MainTab::RequestHeaders),
            Self::Main(MainTab::ResponseData) => Self::Main(MainTab::RequestBody),
            Self::Main(MainTab::ResponseBody) => Self::Main(MainTab::ResponseData),
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
