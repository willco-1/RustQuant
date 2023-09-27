// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{Powi, Variable};
use nalgebra::DVector;
use statrs::function::erf;
use thiserror::Error;
/// Activation functions.
pub trait ActivationFunction {
    /// Applies the sigmoid function to the input.
    fn sigmoid(&self) -> Self;
    /// Applies the identity function to the input.
    fn identity(&self) -> Self;
    /// Applies the logistic function to the input.
    ///
    /// Note (for logistic regression):
    /// sigmoid(x) = 1 / (1 + exp(-x)) = exp(x) / (exp(x) + 1)    
    /// mu(x) = E[Y | X] = P(Y = 1 | X) = sigmoid(w^T x)
    fn logistic(&self) -> Self;
    /// Applies the rectified linear unit function to the input.
    fn relu(&self) -> Self;
    /// Applies the gaussian error linear unit function to the input.
    fn gelu(&self) -> Self;
    /// Applies the hyperbolic tangent function to the input.
    fn tanh(&self) -> Self;
    /// Applies the softplus function to the input.
    fn softplus(&self) -> Self;
    /// Applies the gaussian function to the input.
    fn gaussian(&self) -> Self;
}

impl ActivationFunction for Variable<'_> {
    #[inline]
    fn sigmoid(&self) -> Self {
        ((-(*self)).exp() + 1_f64).recip()
    }

    #[inline]
    fn identity(&self) -> Self {
        *self
    }

    #[inline]
    fn logistic(&self) -> Self {
        ((-(*self)).exp() + 1_f64).recip()
    }

    #[inline]
    fn relu(&self) -> Self {
        (*self + (*self).abs()) / 2_f64
    }

    #[inline]
    fn gelu(&self) -> Self {
        0.5_f64 * (*self) * (1.0 + ((*self) / 2_f64.sqrt()).erf())
    }

    #[inline]
    fn tanh(&self) -> Self {
        (*self).tanh()
    }

    #[inline]
    fn softplus(&self) -> Self {
        (1.0 + self.exp()).ln()
    }

    #[inline]
    fn gaussian(&self) -> Self {
        (-self.powi(2)).exp()
    }
}

impl ActivationFunction for f64 {
    #[inline]
    fn sigmoid(&self) -> Self {
        1.0 / (1.0 + (-self).exp())
    }

    #[inline]
    fn identity(&self) -> Self {
        *self
    }

    #[inline]
    fn logistic(&self) -> Self {
        1.0 / (1.0 + (-self).exp())
    }

    #[inline]
    fn relu(&self) -> Self {
        if *self > 0.0 {
            *self
        } else {
            0.0
        }
    }

    #[inline]
    fn gelu(&self) -> Self {
        0.5 * self * (1.0 + erf::erf(self / 2_f64.sqrt()))
    }

    #[inline]
    fn tanh(&self) -> Self {
        f64::tanh(*self)
    }

    #[inline]
    fn softplus(&self) -> Self {
        (1.0 + self.exp()).ln()
    }

    #[inline]
    fn gaussian(&self) -> Self {
        (f64::powi(-self, 2)).exp()
    }
}

impl ActivationFunction for DVector<f64> {
    #[inline]
    fn sigmoid(&self) -> Self {
        self.map(|x| x.sigmoid())
    }

    #[inline]
    fn identity(&self) -> Self {
        self.clone()
    }

    #[inline]
    fn logistic(&self) -> Self {
        self.map(|x| x.logistic())
    }

    #[inline]
    fn relu(&self) -> Self {
        self.map(|x| x.relu())
    }

    #[inline]
    fn gelu(&self) -> Self {
        self.map(|x| x.gelu())
    }

    #[inline]
    fn tanh(&self) -> Self {
        self.map(|x| x.tanh())
    }

    #[inline]
    fn softplus(&self) -> Self {
        self.map(|x| x.softplus())
    }

    #[inline]
    fn gaussian(&self) -> Self {
        self.map(|x| x.gaussian())
    }
}
