use std::io::Write;
use std::str::FromStr;

struct Args(Vec<u64>);

impl Args {
    fn to_tuple(self) -> (u64, u64) {
        // defaults
        let work_t = 15;
        let rest_t = 5;

        let vec = self.0;
        match vec.len() {
             0 => (work_t, rest_t),
             1 => (vec[0], rest_t),
             2 => (vec[0], vec[1]),
             _ => panic!("Error: Too many values"),
        }
    }
}

fn main() {
    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(u64::from_str(&arg)
            .expect("Error: Unknown argument"));
    }

    if args.len() > 2 {
        writeln!(std::io::stderr(),
                 "Usage: 20min n (working time) n (time for rest)").unwrap();
        std::process::exit(1);
    }

    let (work_t, rest_t) = Args(args).to_tuple();
    println!("work_t {}, rest_t {}", work_t, rest_t);
}


#[test]
fn test_args_to_tuple() {
    let mut v;

    v = vec![];
    assert_eq!((15, 5), Args(v).to_tuple());

    v = vec![10];
    assert_eq!((10, 5), Args(v).to_tuple());

    v = vec![20, 30];
    assert_eq!((20, 30), Args(v).to_tuple());

    v = vec![30, 40, 50];
    let res = std::panic::catch_unwind(|| Args(v).to_tuple());
    assert!(res.is_err());
}
