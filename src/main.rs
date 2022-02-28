use anyhow::{anyhow, bail, Result};
use getopts::Options;
use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let text = match parse_args(&args) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    println!("{}", pangu::spacing(&text));
}

fn parse_args(args: &[String]) -> Result<String> {
    let mut opts = Options::new();

    opts.optopt("f", "", "read text from file", "FILE");
    opts.optflag("v", "version", "Print the version");
    opts.optflag("h", "help", "Print this help menu");

    let usage = opts.usage(&format!("Usage: {} [options]", args[0]));

    let matches = opts
        .parse(&args[1..])
        .map_err(|err| anyhow!("{err}\n\n{usage}"))?;

    if matches.opt_present("v") {
        bail!("{}", env!("CARGO_PKG_VERSION"));
    }

    if matches.opt_present("h") {
        bail!("{usage}");
    }

    let file = matches.opt_str("f");
    let mut free = matches.free.into_iter();

    let text = match (file, free.next(), free.next()) {
        (Some(file), None, None) => fs::read_to_string(file)?,
        (None, Some(text), None) => text,
        _ => bail!("Bad input\n\n{usage}"),
    };

    Ok(text)
}
