use clap::{Parser, arg, command};

#[derive(Parser, Debug)]
#[command(name = "Curl++")]
#[command(about = "An HTTP test client TUI")]
struct Args {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<String>,
    #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
    body: Vec<String>,
}
