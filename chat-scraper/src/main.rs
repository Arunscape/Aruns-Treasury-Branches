use lazy_static::lazy_static;
use linemux::MuxedLines;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let f: &'static str = "/home/arunscape/Downloads/mc118/data/logs/latest.log";

    let mut lines = MuxedLines::new()?;

    lines.add_file(&f).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        // dbg!(line.line());
        parse_chat_msg(line.line());
    }

    Ok(())
}

struct Command {
    username: String,
    action: Action,
}

enum Action {
    CreateAccount(String),
    CloseAccount(String),
    Deposit(Transaction),
    Withdraw(Transaction),
    Transfer(Transaction),
}

struct Transaction {
    from: String,
    to: String,
    amount: usize,
}

fn parse_chat_msg(line: &str) {
    // [05:49:01] [Async Chat Thread - #11/INFO]: <Arunscape> hello

    if let Some((time, msg)) = line.split_once(": ") {
        // dbg!(&time);
        if !is_chat(time) {
            return;
        }

        // it matched
        // dbg!(&msg);

        let (username, msg) = msg.split_once(" ").unwrap();

        if !msg.starts_with("atb ") {
            return;
        }

        let username = &username[1..username.len() - 1];

        println!("ok, parsing message");

        dbg!(&username);
        dbg!(&msg);
    }
}

fn is_chat(s: &str) -> bool {
    // prevent compiling regex in a loop
    dbg!(&s);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[Async Chat Thread - #\d+/INFO\]").unwrap();
    }
    RE.is_match(s)
}
