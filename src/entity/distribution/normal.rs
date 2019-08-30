use lazycell::LazyCell;
use super::args::Args;
use rand_distr::Distribution;

#[derive(Debug,Clone)]
pub struct Normal(LazyCell<f64>);

impl Normal {
    const DEFAULT_ROUNDING: i8 = 6;

    pub fn new() -> Normal {
        Normal( LazyCell::new() )
    }

    pub fn get(&self, args: NormalArgs) -> f64 {
        *self.0.borrow_with(|| Normal::generate(args.mean, args.stddev))
    }

    fn generate(mean: f64, stddev: f64) -> f64 {
        let dist = rand_distr::Normal::new(mean, stddev).unwrap();
        let val = dist.sample(&mut rand::thread_rng());
        math::round::floor(val, Normal::DEFAULT_ROUNDING)
    }
}

#[derive(Debug)]
pub struct NormalArgs {
    pub mean: f64,
    pub stddev: f64
}

impl Args for NormalArgs {
    fn help() -> &'static str {
        ""
    }

    fn default() -> NormalArgs {
        NormalArgs { mean: 0f64, stddev: 1.0f64 }
    }

    fn parse(args: &String) -> Option<NormalArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [mean, stddev] = arg_vec[..] {
            let mean_val: Option<f64> = mean.parse::<f64>().ok();
            let stddev_val: Option<f64> = stddev.parse::<f64>().ok()
                .and_then(|val: f64| match val < 0.0 {
                    true => None,
                    false => Some(val)
                });
            if mean_val.is_some() && stddev_val.is_some() {
                return Some(NormalArgs { mean: mean_val.unwrap(), stddev: stddev_val.unwrap() });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_args_valid() {
        let parsed_args = NormalArgs::parse(&"1,2".to_owned()).unwrap();
        assert_eq!(parsed_args.mean, 1.0);
        assert_eq!(parsed_args.stddev, 2.0);
    }

    #[test]
    fn parse_invalid_args_returns_none() {
        let invalid_args = vec!["", "1", "2", "string", "1,2,3"];
        for arg in invalid_args {
            let parsed_args = NormalArgs::parse(&arg.to_owned());
            assert!(parsed_args.is_none());
        }
    }
}