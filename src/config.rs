pub struct Config {
    pub params: Vec<f64>,
}

impl Config {
    pub fn to_tuple(self) -> (f64, f64) {
        // defaults (seconds)
        let work_t = 15.0 * 60.0;
        let rest_t = 5.0 * 60.0;

        let vec = self.params;
        match vec.len() {
             0 => (work_t, rest_t),
             1 => (vec[0], rest_t),
             2 => (vec[0], vec[1]),
             _ => panic!("Error: Too many values"),
        }
    }
}

#[cfg(test)]
mod config_test {
    use std::panic;
    use super::*;

    #[test]
    fn test_config_to_tuple() {
        let mut c;

        c = Config { params: vec![] };
        assert_eq!((900.0, 300.0), c.to_tuple());

        c = Config { params: vec![10.0] };
        assert_eq!((10.0, 300.0), c.to_tuple());

        c = Config { params: vec![20.0, 30.0] };
        assert_eq!((20.0, 30.0), c.to_tuple());

        c = Config { params: vec![30.0, 40.0, 50.0] };
        let res = panic::catch_unwind(|| c.to_tuple());
        assert!(res.is_err());
    }
}
