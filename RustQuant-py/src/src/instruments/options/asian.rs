// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::OffsetDateTime;

use crate::{
    statistics::distributions::{gaussian::*, Distribution},
    time::{DayCountConvention, DayCounter},
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Type of Asian option (fixed or floating strike).
#[derive(Debug, Clone, Copy)]
pub enum PyAsianStrike {
    /// Floating strike Asian option.
    /// Payoffs:
    /// - Call: `max(S_T - A, 0)`
    /// - Put: `max(A - S_T, 0)`
    Floating,
    /// Fixed strike Asian option.
    /// Payoffs:
    /// - Call: `max(A - K, 0)`
    /// - Put: `max(K - A, 0)`
    Fixed,
}

/// Method of averaging (arithmetic or geometric, and continuous or discrete).
#[derive(Debug, Clone, Copy)]
pub enum AveragingMethod {
    /// Arithmetic Asian option with discrete averaging.
    ArithmeticDiscrete,
    /// Arithmetic Asian option with continuous averaging.
    ArithmeticContinuous,
    /// Geometric Asian option with discrete averaging.
    GeometricDiscrete,
    /// Geometric Asian option with continuous averaging.
    GeometricContinuous,
}

/// Asian Option struct.
#[derive(Debug, Clone, Copy)]
pub struct PyAsianOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `K` - Strike price.
    pub strike_price: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `v` - Volatility parameter.
    pub volatility: f64,
    /// `q` - Dividend rate.
    pub dividend_rate: f64,
    /// `valuation_date` - Valuation date.
    pub valuation_date: Option<OffsetDateTime>,
    /// `expiry_date` - Expiry date.
    pub expiry_date: OffsetDateTime,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl AsianOption {
    /// New Asian Option
    pub fn new(
        initial_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        dividend_rate: f64,
        valuation_date: Option<OffsetDateTime>,
        expiry_date: OffsetDateTime,
    ) -> Self {
        Self {
            initial_price,
            strike_price,
            risk_free_rate,
            volatility,
            dividend_rate,
            valuation_date,
            expiry_date,
        }
    }

    /// Geometric Continuous Average-Rate Price
    pub fn price_geometric_average(&self) -> (f64, f64) {
        let S = self.initial_price;
        let K = self.strike_price;
        // let T = self.time_to_maturity;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

        // Compute time to maturity.
        let T = match self.valuation_date {
            Some(valuation_date) => DayCounter::day_count_factor(
                valuation_date,
                self.expiry_date,
                &DayCountConvention::Actual365,
            ),
            None => DayCounter::day_count_factor(
                OffsetDateTime::now_utc(),
                self.expiry_date,
                &DayCountConvention::Actual365,
            ),
        };

        let v_a = v / 3_f64.sqrt();
        let b = r - q;
        let b_a = 0.5 * (b - v * v / 6.0);

        let d1 = ((S / K).ln() + (b_a + 0.5 * v_a * v_a) * T) / (v_a * (T).sqrt());
        let d2 = d1 - v_a * (T).sqrt();

        let N = Gaussian::default();

        let c = S * ((b_a - r) * T).exp() * N.cdf(d1) - K * (-r * T).exp() * N.cdf(d2);
        let p = -S * ((b_a - r) * T).exp() * N.cdf(-d1) + K * (-r * T).exp() * N.cdf(-d2);

        (c, p)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_asian_geometric() {
        let expiry_date = OffsetDateTime::now_utc() + Duration::days(92);

        let AsianOption = AsianOption {
            initial_price: 80.0,
            strike_price: 85.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            valuation_date: None,
            expiry_date,
            dividend_rate: -0.03,
        };

        let prices = AsianOption.price_geometric_average();

        // Value from Haug's book.
        assert_approx_equal!(prices.1, 4.6922, 0.0001);
    }
}
