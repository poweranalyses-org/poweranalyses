use dist::Dist;
use dist::NoncentralChisq;
use dist::NoncentralF;
use dist::NoncentralT;
use roots::find_root_regula_falsi;
use roots::SimpleConvergency;
use serde_json::Value;

/// Supertype for all test types.
///
/// See the G*Power 3 paper for the equations for the distribution parameters
/// (https://doi.org/10.3758/BF03193146).
pub enum TestKind {
    OneSampleTTest,
    IndependentSamplesTTest,
    DeviationFromZeroMultipleRegression {
        /// Number of predictors (#A).
        n_predictors: i64,
    },
    GoodnessOfFitChisqTest {
        /// Degrees of freedom.
        df: i64,
    },
    /// Multiple regression: increase of R^2.
    IncreaseMultipleRegression {
        /// Total number of predictors (#A + #B).
        rho: i64,
        /// Number of tested predictors (#B).
        q: i64,
    },
}

#[derive(Clone, Debug)]
pub enum Tail {
    OneSided,
    TwoSided,
}

fn parse_i64(data: &Value, field: &str) -> Result<i64, String> {
    let value = match data.get(field) {
        Some(value) => value,
        None => return Err(format!("Missing field: {}", field)),
    };
    let value: &str = value
        .as_str()
        .expect("{field} could not be converted to a str");
    let value: i64 = value
        .parse()
        .expect("{field} could not be converted to an integer");
    Ok(value)
}

impl Tail {
    pub fn from_json(data: &Value) -> Option<Tail> {
        let tail: i64 = parse_i64(data, "tail").unwrap();
        match tail {
            1 => Some(Tail::OneSided),
            2 => Some(Tail::TwoSided),
            _ => None,
        }
    }
}

impl TestKind {
    pub fn from_str(text: &str, data: &Value) -> Result<TestKind, String> {
        match text {
            "oneSampleTTest" => Ok(TestKind::OneSampleTTest),
            "independentSamplesTTest" => Ok(TestKind::IndependentSamplesTTest),
            "deviationFromZeroMultipleRegression" => {
                let n_predictors = parse_i64(data, "nPredictors").unwrap();
                Ok(TestKind::DeviationFromZeroMultipleRegression { n_predictors })
            }
            "goodnessOfFitChisqTest" => {
                let df = parse_i64(data, "df").unwrap();
                Ok(TestKind::GoodnessOfFitChisqTest { df })
            }
            "increaseMultipleRegression" => {
                let rho = parse_i64(data, "rho").unwrap();
                let q = parse_i64(data, "q").unwrap();
                Ok(TestKind::IncreaseMultipleRegression { rho, q })
            }
            _ => Err(format!("Unknown test: {}", text)),
        }
    }

    fn alternative_distribution(&self, n: f64, es: f64) -> Dist {
        match self {
            TestKind::OneSampleTTest => Box::new(NoncentralT::new(n - 1.0, n.sqrt() * es)),
            TestKind::IndependentSamplesTTest => {
                let v = n - 2.0; // n1 + n2 - 2
                Box::new(NoncentralT::new(v, (n / 2.0).sqrt() * es))
            }
            TestKind::DeviationFromZeroMultipleRegression { n_predictors } => {
                Box::new(NoncentralF::new(
                    *n_predictors as f64,
                    n - (*n_predictors as f64) - 1.0,
                    es.powi(2) * n,
                ))
            }
            TestKind::GoodnessOfFitChisqTest { df } => {
                Box::new(NoncentralChisq::new(*df as f64, es.powi(2) * n))
            }
            TestKind::IncreaseMultipleRegression { rho, q } => Box::new(NoncentralF::new(
                *q as f64,
                n - (*rho as f64) - 1.0,
                es.powi(2) * n,
            )),
        }
    }

    fn null_distribution(&self, n: f64, es: f64) -> Dist {
        self.alternative_distribution(n, es).central_distribution()
    }

    pub fn n(&self, tail: Tail, alpha: f64, power: f64, es: f64) -> i64 {
        let f = |n| self.alpha(tail.clone(), n, power, es) - alpha;
        let mut conv = SimpleConvergency {
            eps: 0.0001f64,
            max_iter: 500,
        };
        let step_size = 20;
        // There is probably a better way to do this, but it works.
        for lower in (0..1000).step_by(step_size) {
            let upper = lower + step_size;
            let root = find_root_regula_falsi(lower as f64, upper as f64, f, &mut conv);
            let n = root.unwrap_or(-111.0);
            if n == -111.0 || n.is_nan() {
                continue;
            }
            return n.round() as i64;
        }
        return -111;
    }

    pub fn alpha(&self, tail: Tail, n: f64, power: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let critical_value = d1.quantile(power, false);
        let right_tail = d0.cdf(critical_value, false);
        match tail {
            Tail::OneSided => right_tail,
            Tail::TwoSided => 2.0 * right_tail,
        }
    }

    pub fn power(&self, tail: Tail, n: f64, alpha: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let right_tail = match tail {
            Tail::OneSided => alpha,
            Tail::TwoSided => alpha / 2.0,
        };
        let critical_value = d0.quantile(right_tail, false);
        d1.cdf(critical_value, false)
    }

    pub fn es(&self, tail: Tail, n: f64, alpha: f64, power: f64) -> f64 {
        let f = |es| self.alpha(tail.clone(), n, power, es) - alpha;
        let mut conv = SimpleConvergency {
            eps: 0.0001f64,
            max_iter: 500,
        };
        let root = find_root_regula_falsi(0.001f64, 8f64, f, &mut conv);
        root.unwrap_or(-111.0)
    }
}
