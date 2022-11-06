use crate::dist::*;

use roots::*;

trait StatisticalTest {
    fn null_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution>;
    fn alternative_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution>;
    fn power(&self, tail: i32, es: f64, alpha: f64, n: f64) -> f64 {
        let d0 = self.null_distribution(es, n);
        let d1 = self.alternative_distribution(es, n);
        let right_tail = if tail == 1 { alpha } else { alpha / 2.0 };
        let critical_value = d0.quantile(right_tail, false);
        let beta = d1.cdf(critical_value, false);
        return beta;
    }
    fn alpha(&self, tail: i32, es: f64, power: f64, n: f64) -> f64 {
        let d0 = self.null_distribution(es, n);
        let d1 = self.alternative_distribution(es, n);
        let critical_value = d1.quantile(power, false);
        let right_tail = d0.cdf(critical_value, false);
        return if tail == 1 { right_tail } else { 2.0 * right_tail };
    }
    fn es(&self, tail: i32, alpha: f64, power: f64, n: f64) -> f64 {
        let f = | es | { self.alpha(tail, es, power, n) - alpha };
        let mut convergency = SimpleConvergency { eps: 0.00001f64, max_iter: 40 };
        return find_root_brent(0f64, 1000f64, &f, &mut convergency).unwrap();
    }
    fn n(&self, tail: i32, alpha: f64, power: f64, es: f64) -> f64 {
        let f = | n | {
            self.alpha(tail, es, power, n) - alpha
        };
        let mut convergency = SimpleConvergency { eps: 0.00001f64, max_iter: 40 };
        return find_root_brent(2f64, 10000f64, &f, &mut convergency).unwrap();
    }
}

struct OneSampleTTest {}

impl StatisticalTest for OneSampleTTest {
    fn null_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: n - 1.0, lambda: 0.0 })
    }
    fn alternative_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: n - 1.0, lambda: n.sqrt() * es });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let es = 0.5;
        let alpha = 0.05;
        let power = 0.95;
        let n = 50.0;
        assert_eq!(OneSampleTTest{}.alpha(2, es, power, n), 0.06731683009994659);
        assert_eq!(OneSampleTTest{}.alpha(1, es, power, n), 0.03365841504997329);
        assert_eq!(OneSampleTTest{}.power(2, es, alpha, n), 0.9338975528614741);
        assert_eq!(OneSampleTTest{}.power(1, es, alpha, n), 0.9672067458263426);

        assert_eq!(OneSampleTTest{}.es(2, alpha, power, n), 0.5201250999158732);
        assert_eq!(OneSampleTTest{}.es(1, alpha, power, n), 0.4718255595737365);
        assert_eq!(OneSampleTTest{}.n(2, alpha, power, es), 53.94061366252956);
        assert_eq!(OneSampleTTest{}.n(1, alpha, power, es), 44.68070848218932);
    }
}
