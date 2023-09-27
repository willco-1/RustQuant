// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the `Gradient` trait.
//! Each implementation of `wrt` returns the chosen partial derivatives.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GRADIENT TRAIT AND IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Return the derivative/s *with-respect-to* the chosen variables.
/// This allows you to get the gradient of a function with respect to
/// any selection of variables, i.e.
///     - a single variable,
///     - a subset of the variables,
///     - or all of the variables.
pub trait Gradient<IN, OUT> {
    /// Returns the derivative/s *with-respect-to* the chosen variables.
    fn wrt(&self, variables: IN) -> OUT;
}

/// `wrt` a single variable.
impl<'v> Gradient<&Variable<'v>, f64> for Vec<f64> {
    #[inline]
    fn wrt(&self, variable: &Variable) -> f64 {
        self[variable.index]
    }
}

/// `wrt` a borrowed vector of variables.
impl<'v> Gradient<&Vec<Variable<'v>>, Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, variables: &Vec<Variable<'v>>) -> Vec<f64> {
        variables.iter().map(|&var| self[var.index]).collect()
    }
}

/// `wrt` a borrowed slice of variables.
impl<'v> Gradient<&[Variable<'v>], Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, variables: &[Variable<'v>]) -> Vec<f64> {
        variables.iter().map(|&var| self[var.index]).collect()
    }
}

/// `wrt` an array of variables.
impl<'v, const N: usize> Gradient<[Variable<'v>; N], Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, variables: [Variable<'v>; N]) -> Vec<f64> {
        variables.iter().map(|&var| self[var.index]).collect()
    }
}

/// `wrt` a borrowed array of variables.
impl<'v, const N: usize> Gradient<&[Variable<'v>; N], Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, variables: &[Variable<'v>; N]) -> Vec<f64> {
        variables.iter().map(|&var| self[var.index]).collect()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS SECTION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_gradient {
    use crate::{assert_approx_equal, autodiff::*};

    #[test]
    fn test_borrowed_vector() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = x * y;

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, 1e-15);
        assert_eq!(grad.wrt(&vec![x, y]), vec![2.0, 1.0]);
    }

    #[test]
    fn test_borrowed_slice() {
        let g = Graph::new();

        let v: Vec<_> = vec![g.var(1.0), g.var(2.0)];

        let z = v[0] * v[1];

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, 1e-15);
        assert_eq!(grad.wrt(&v[..]), vec![2.0, 1.0]);
    }

    #[test]
    fn test_array() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = x * y;

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, 1e-15);
        assert_eq!(grad.wrt([x, y]), vec![2.0, 1.0]);
    }

    #[test]
    fn test_borrowd_array() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = x * y;

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, 1e-15);
        assert_eq!(grad.wrt(&[x, y]), vec![2.0, 1.0]);
    }

    #[test]
    fn x_times_y_plus_sin_x() {
        let g = Graph::new();

        let x = g.var(69.0);
        let y = g.var(420.0);

        let z = x * y + x.sin();

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 28979.885215, 1e-6);
        assert_approx_equal!(grad.wrt(&x), y.value + x.value.cos(), 1e-15);
        assert_approx_equal!(grad.wrt(&y), x.value, 1e-15);
    }

    #[test]
    fn x_times_y_plus_tan_x() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = x * y + x.tan();

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 3.5574077246549, 1e-14);
        assert_approx_equal!(grad.wrt(&x), 5.425_518_820_814_76, 1e-15);
        assert_approx_equal!(grad.wrt(&y), 1.0, 1e-15);
    }

    #[test]
    fn cosh_x_times_y() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = (x * y).cosh();

        let grad = z.accumulate();

        println!("{}", grad.wrt(&x));

        assert_approx_equal!(z.value, 3.762_195_691_083_631_4, 1e-10);
        assert_approx_equal!(grad.wrt(&x), 7.253_720_815_694_037, 1e-10);
        assert_approx_equal!(grad.wrt(&y), 3.62686040784701, 1e-10);
    }

    #[test]
    fn cosh_xy_div_tanh_x_times_sinh_y() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = (x * y).cosh() / (x.tanh() * y.sinh());

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 1.3620308304831552, 1e-8);
        assert_approx_equal!(grad.wrt(&x), 1.874_990_751_363_869_7, 1e-15);
        assert_approx_equal!(grad.wrt(&y), -0.099_819_345_045_613_27, 1e-15);
    }

    #[test]
    fn test_block_assign() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let f = {
            let z = x.sin() + y.tan();
            z.exp()
        };

        let grad = f.accumulate();

        println!("Grad wrt x = 1.0: \t{}", grad.wrt(&x));
        println!("Grad wrt y = 2.0: \t{}", grad.wrt(&y));

        assert_approx_equal!(grad.wrt(&x), 0.140_971_808_425_461_7, 1e-15);
        assert_approx_equal!(grad.wrt(&y), 1.506_614_888_597_196_4, 1e-15);
    }

    #[test]
    fn test_closure() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        let z = || (x * y).cosh() / (x.tanh() * y.sinh());

        let grad = z().accumulate();

        assert_approx_equal!(z().value, 1.3620308304831552, 1e-8);
        assert_approx_equal!(grad.wrt(&x), 1.874_990_751_363_869_7, 1e-15);
        assert_approx_equal!(grad.wrt(&y), -0.099_819_345_045_613_27, 1e-15);
    }

    #[test]
    fn test_function_input() {
        fn diff_fn<'v>(params: &[Variable<'v>], data: &[f64]) -> Variable<'v> {
            params[0].powf(params[1]) + data[0].sin() - params[2].asinh() / data[1]
        }

        let graph = Graph::new();
        let params = graph.vars(&[3.0, 2.0, 1.0]);
        let data = [1., 2.];
        let result = diff_fn(&params, &data);
        let gradients = result.accumulate();
        println!("{:?}", gradients.wrt(&params));
        println!("{:?}", gradients);
    }
}
