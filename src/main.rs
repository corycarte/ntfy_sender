use core::fmt;
use std::io::{self, BufRead};

use atty::Stream;
use reqwest;
use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(long)]
    title: Option<String>,

    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long)]
    endpoint: String,

    #[clap(value_enum, default_value_t=NtfyPriority::Default)]
    priority: NtfyPriority,

    #[arg(short, long, default_value_t=String::new())]
    tags: String,
}

#[derive(ValueEnum, Clone, Debug)]
enum NtfyPriority {
    Min,
    Low,
    Default,
    High,
    Max,
}

impl fmt::Display for NtfyPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NtfyPriority::Min  => write!(f, "min"),
            NtfyPriority::Low  => write!(f, "low"),
            NtfyPriority::High => write!(f, "high"),
            NtfyPriority::Max  => write!(f, "max"),
            _                  => write!(f, "default"),
        }
    }
}

fn read_std_in() -> io::Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "This isn't right"));
    }

    let message = io::stdin().lock().lines().fold("".to_string(), |acc, line| {
        acc + &line.unwrap()
    });

    return Ok(message);
}


#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let message: String;
    let title: String;

    match read_std_in() {
        Ok(m) => {
            message = m.to_string();
            dbg!("Using standard in");
        },
        Err(_e) => {
            message = args.message.unwrap();
            dbg!("Using cli args");
        }
    }

    if let Some(t) = args.title {
        title = t;
    } else {
        title = format!("New {} priority notification", args.priority);
    }

    println!("Sending {message} to {} with priority {}", args.endpoint, args.priority);

    let _ = reqwest::Client::new()
        .post(args.endpoint)
        .body(message)
        .header("Priority", args.priority.to_string())
        .header("Title", title)
        .header("Tags", args.tags)
        .send()
        .await;

}
