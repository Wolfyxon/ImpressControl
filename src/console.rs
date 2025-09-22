use std::{io::{self, BufRead, Write}, process::exit};

const MAX_MESSAGE_LEN: usize = 128;

pub fn read_line() -> String {
    let stdin = io::stdin();
    let mut res = String::new();

    stdin.lock().read_line(&mut res).expect("stdin read failed");

    res.trim_end().to_string()
}

pub fn ask(msg: impl Into<String>) -> String {
    print(format!("{}: ", msg.into()));
    read_line()
}

pub fn ask_default(msg: impl Into<String>, default: impl Into<String>) -> String {
    let def = default.into();
    let res = ask(format!("{} (default: {})", msg.into(), &def));

    if res.is_empty() {
        return def;
    } else {
        return res;
    }
}

pub fn print(msg: impl Into<String>) {
    let mut stdout = io::stdout();
    print!("{}", msg.into());

    stdout.flush().expect("Failed to flush stdout");
}

pub fn confirm(msg: impl Into<String>) -> bool {
    return ask(format!("{} [y/n]", msg.into())).to_lowercase() == "y";
}

pub fn confirm_or_exit(msg: impl Into<String>) {
    if !confirm(msg) {
        exit(0);
    }
}

pub fn debug_msg_string(string: String) -> String {
    let mut res = string.split("\n").next().unwrap().to_string();

    if res.len() > MAX_MESSAGE_LEN {
        res = format!("{}...", res[..MAX_MESSAGE_LEN].to_string());
    }

    res.replace("\n", "\\n")
}
