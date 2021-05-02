//! A command line working timer `20min` (twenty-minutes).
//!
//! # Examples
//!
//! ```zsh
//! % 20min 15,5
//! % 20min 0.5 5
//! ```
extern crate structopt;

use std::process::exit;

use structopt::StructOpt;

use twenty_minutes::{config::Config, ticker::tick, split_arg, report};

/// Command line options.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "20min",
    about = "\nA command line working timer. Run with arguments `15,5` \
             (minutes) or OPTIONS \nlike `--work 15 --rest 15`."
)]
struct Opts {
    /// Working time
    #[structopt(short = "w", long = "work", default_value = "15")]
    work: String,

    /// Time of rest
    #[structopt(short = "r", long = "rest", default_value = "5")]
    rest: String,
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // detect argument type ("20,3" or "--work 5 --rest 0.1")
    let mut use_opts: bool = false;
    let o = [
        "-w",
        "--work",
        "-r",
        "--rest",
        "-h",
        "--help",
        "-V",
        "--version",
    ];
    for (_, opt) in o.iter().enumerate() {
        if args.contains(&(opt.to_string())) {
            use_opts = true;
            break;
        }
    }

    if use_opts {
        let opts = Opts::from_args();
        args = vec![opts.work, opts.rest];
    }

    if args.len() == 1 {
        // handle single string input like: "15,5"
        if let Some(v) = split_arg(&args[0]) {
            args = v;
        }
    }

    let c = Config::new(args);
    if c.params.len() > 2 {
        eprintln!("Usage: 20min n (working time) n (time for rest)");
        exit(1);
    }

    let (work_t, rest_t) = c.to_tuple();

    tick(work_t, "Working");
    report(work_t);

    tick(rest_t, "Resting");
    report(rest_t);
}
