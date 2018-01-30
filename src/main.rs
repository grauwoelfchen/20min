use std::io::Write;
use std::str::FromStr;


fn main() {
    let mut work_t: u64 = 15;
    let mut rest_t: u64 = 5;

    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if args.len() == 0 {
        // pass (use default)
    } else if args.len() == 1 {
        work_t = args[0];
    } else if args.len() == 2 {
        work_t = args[0];
        rest_t = args[1];
    } else {
        writeln!(std::io::stderr(),
                 "Usage: 20min n (working time) n (time for rest)").unwrap();
        std::process::exit(1);
    }

    println!("work_t {}, rest_t {}", work_t, rest_t);
}
