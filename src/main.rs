use getopts::Options;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = match parse(args) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{}", pangu::spacing(&text));
}

fn parse(args: Vec<String>) -> Result<String, String> {
    let mut opts = Options::new();

    opts.optopt("f", "", "read text from file", "FILENAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts
        .parse(&args[1..])
        .or_else(|e| return Err(e.to_string()))?;

    if matches.opt_present("h") {
        return Err(opts.usage(&format!("Usage: {} [options] [text]", args[0].clone())));
    }

    let file = matches.opt_str("f");

    if file.is_none() && matches.free.is_empty() {
        return Err(String::from("No input"));
    } else if file.is_some() && !matches.free.is_empty() {
        let mut free = String::new();

        for arg in matches.free {
            free.push_str(&arg);
        }

        return Err(format!("Unrecognized argument: {}", free));
    } else if file.is_none() && matches.free.len() > 1 {
        let mut free = String::new();

        matches
            .free
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx > &0)
            .for_each(|(_, arg)| free.push_str(arg));

        return Err(format!("Unrecognized argument: {}", free));
    }

    let text = match file {
        Some(f) => fs::read_to_string(f).or_else(|e| return Err(e.to_string()))?,
        None => matches.free[0].clone(),
    };

    Ok(text)
}
