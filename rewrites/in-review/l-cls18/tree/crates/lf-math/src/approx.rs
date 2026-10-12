//! Tolerance comparisons for floats and the maths types.
//!
//! Exact `==` on floats is right only for values that were copied, never
//! computed. Everything else compares with a tolerance:
//!
//! - [`ApproxEq::abs_diff_eq`]: every component differs by at most `eps`.
//!   Right for values of a known scale (unit vectors, rotations).
//! - [`ApproxEq::relative_eq`]: every component differs by at most `eps`
//!   absolutely *or* `rel` relative to the larger magnitude. Right when
//!   magnitudes vary (world positions, products of matrices).
//!
//! Non-finite components never compare equal, except two infinities of the
//! same sign. Quaternions compare component-wise; use
//! [`Quat::same_rotation`] to treat `q` and `-q` as equal.

use crate::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// Default absolute tolerance of [`ApproxEq::approx_eq`].
pub const DEFAULT_EPS: f32 = 1.0e-5;

/// Default relative tolerance of [`ApproxEq::approx_eq`].
pub const DEFAULT_REL: f32 = 1.0e-5;

/// Tolerance comparison, component by component.
pub trait ApproxEq {
    /// True when every pair of components differs by at most `eps`.
    fn abs_diff_eq(&self, other: &Self, eps: f32) -> bool;

    /// True when every pair of components is within `eps` absolutely or
    /// within `rel` times the larger magnitude.
    fn relative_eq(&self, other: &Self, eps: f32, rel: f32) -> bool;

    /// [`ApproxEq::relative_eq`] with [`DEFAULT_EPS`] and [`DEFAULT_REL`].
    fn approx_eq(&self, other: &Self) -> bool {
        self.relative_eq(other, DEFAULT_EPS, DEFAULT_REL)
    }
}

impl ApproxEq for f32 {
    fn abs_diff_eq(&self, other: &f32, eps: f32) -> bool {
        if self.is_infinite() || other.is_infinite() {
            return self == other;
        }
        (self - other).abs() <= eps
    }

    fn relative_eq(&self, other: &f32, eps: f32, rel: f32) -> bool {
        if self.is_infinite() || other.is_infinite() {
            return self == other;
        }
        let diff = (self - other).abs();
        diff <= eps || diff <= rel * self.abs().max(other.abs())
    }
}

/// Implements [`ApproxEq`] by comparing the `to_*_array` components.
macro_rules! impl_approx_by_array {
    ($t:ty, $to_array:ident) => {
        impl ApproxEq for $t {
            fn abs_diff_eq(&self, other: &Self, eps: f32) -> bool {
                self.$to_array()
                    .iter()
                    .zip(other.$to_array().iter())
                    .all(|(a, b)| a.abs_diff_eq(b, eps))
            }

            fn relative_eq(&self, other: &Self, eps: f32, rel: f32) -> bool {
                self.$to_array()
                    .iter()
                    .zip(other.$to_array().iter())
                    .all(|(a, b)| a.relative_eq(b, eps, rel))
            }
        }
    };
}

impl_approx_by_array!(Vec2, to_array);
impl_approx_by_array!(Vec3, to_array);
impl_approx_by_array!(Vec4, to_array);
impl_approx_by_array!(Mat3, to_cols_array);
impl_approx_by_array!(Mat4, to_cols_array);

impl ApproxEq for Quat {
    fn abs_diff_eq(&self, other: &Quat, eps: f32) -> bool {
        self.to_vec4().abs_diff_eq(&other.to_vec4(), eps)
    }

    fn relative_eq(&self, other: &Quat, eps: f32, rel: f32) -> bool {
        self.to_vec4().relative_eq(&other.to_vec4(), eps, rel)
    }
}

impl Quat {
    /// True when two unit quaternions are the same rotation within `eps`
    /// per component, counting `q` and `-q` as equal.
    #[must_use]
    pub fn same_rotation(&self, other: &Quat, eps: f32) -> bool {
        self.abs_diff_eq(other, eps) || self.abs_diff_eq(&-*other, eps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_rules() {
        assert!(1.0f32.abs_diff_eq(&1.000_001, 1e-5));
        assert!(!1.0f32.abs_diff_eq(&1.001, 1e-5));
        assert!(1.0e6f32.relative_eq(&1.000_001e6, 1e-5, 1e-5));
        assert!(!1.0e6f32.abs_diff_eq(&1.000_1e6, 1e-5));
        assert!(!f32::NAN.approx_eq(&f32::NAN));
        assert!(f32::INFINITY.approx_eq(&f32::INFINITY));
        assert!(!f32::INFINITY.approx_eq(&f32::NEG_INFINITY));
        assert!(!f32::INFINITY.approx_eq(&f32::MAX));
    }

    #[test]
    fn types_compare_component_wise() {
        assert!(Vec3::new(1.0, 2.0, 3.0).approx_eq(&Vec3::new(1.0, 2.0, 3.000_001)));
        assert!(!Vec3::new(1.0, 2.0, 3.0).approx_eq(&Vec3::new(1.0, 2.1, 3.0)));
        assert!(Mat4::IDENTITY.abs_diff_eq(&Mat4::IDENTITY, 0.0));
        assert!(!Mat3::IDENTITY.approx_eq(&Mat3::ZERO));
        let q = Quat::from_rotation_z(0.3);
        assert!(!q.approx_eq(&-q));
        assert!(q.same_rotation(&-q, 1e-6));
        assert!(Vec2::ONE.approx_eq(&Vec2::splat(1.0)));
        assert!(Vec4::W.approx_eq(&Vec4::W));
    }
}
