use serde_json::Value;
use std::iter;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{
    Application,
    state::{HeaderSection, MainTab, Panel, RequestHeaderFocus},
    ui::{components::badge::badge, palette},
};

impl Application {}
