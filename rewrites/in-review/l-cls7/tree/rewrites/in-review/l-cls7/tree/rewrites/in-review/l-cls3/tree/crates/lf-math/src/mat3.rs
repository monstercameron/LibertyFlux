//! [`Mat3`]: a 3x3 column-major matrix (rotations, normal matrices).

use core::ops::{Mul, MulAssign};

use crate::{Mat4, Quat, Vec3};

/// A 3x3 matrix of three [`Vec3`] columns. See the crate docs for the
/// conventions (column vectors, column-major storage) and the GPU layout
/// note (36 bytes; not std140-compatible as it stands).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    /// Columns 0, 1 and 2.
    pub cols: [Vec3; 3],
}

impl Default for Mat3 {
    fn default() -> Self {
        Mat3::IDENTITY
    }
}

impl Mat3 {
    /// The identity.
    pub const IDENTITY: Mat3 = Mat3::from_cols(Vec3::X, Vec3::Y, Vec3::Z);
    /// All zero.
    pub const ZERO: Mat3 = Mat3::from_cols(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);

    /// A matrix from its columns.
    #[must_use]
    pub const fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) -> Mat3 {
        Mat3 { cols: [c0, c1, c2] }
    }

    /// A matrix from nine floats in column-major order.
    #[must_use]
    pub const fn from_cols_array(a: &[f32; 9]) -> Mat3 {
        Mat3::from_cols(
            Vec3::new(a[0], a[1], a[2]),
            Vec3::new(a[3], a[4], a[5]),
            Vec3::new(a[6], a[7], a[8]),
        )
    }

    /// The nine floats in column-major order.
    #[must_use]
    pub const fn to_cols_array(&self) -> [f32; 9] {
        let [c0, c1, c2] = self.cols;
        [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z]
    }

    /// The upper-left 3x3 block of a [`Mat4`] (its linear part).
    #[must_use]
    pub const fn from_mat4(m: &Mat4) -> Mat3 {
        Mat3::from_cols(
            m.cols[0].truncate(),
            m.cols[1].truncate(),
            m.cols[2].truncate(),
        )
    }

    /// A non-uniform scale.
    #[must_use]
    pub const fn from_scale(s: Vec3) -> Mat3 {
        Mat3::from_cols(
            Vec3::new(s.x, 0.0, 0.0),
            Vec3::new(0.0, s.y, 0.0),
            Vec3::new(0.0, 0.0, s.z),
        )
    }

    /// The rotation of a unit quaternion.
    #[must_use]
    pub fn from_quat(q: Quat) -> Mat3 {
        let (x2, y2, z2) = (q.x + q.x, q.y + q.y, q.z + q.z);
        let (xx, yy, zz) = (q.x * x2, q.y * y2, q.z * z2);
        let (xy, xz, yz) = (q.x * y2, q.x * z2, q.y * z2);
        let (wx, wy, wz) = (q.w * x2, q.w * y2, q.w * z2);
        Mat3::from_cols(
            Vec3::new(1.0 - (yy + zz), xy + wz, xz - wy),
            Vec3::new(xy - wz, 1.0 - (xx + zz), yz + wx),
            Vec3::new(xz + wy, yz - wx, 1.0 - (xx + yy)),
        )
    }

    /// A rotation of `angle` radians about a unit `axis`.
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Mat3 {
        Mat3::from_quat(Quat::from_axis_angle(axis, angle))
    }

    /// Row `i` as a vector.
    ///
    /// # Panics
    ///
    /// Panics when `i > 2`.
    #[must_use]
    pub fn row(&self, i: usize) -> Vec3 {
        Vec3::new(self.cols[0][i], self.cols[1][i], self.cols[2][i])
    }

    /// The transpose.
    #[must_use]
    pub fn transpose(&self) -> Mat3 {
        Mat3::from_cols(self.row(0), self.row(1), self.row(2))
    }

    /// The determinant.
    #[must_use]
    pub fn determinant(&self) -> f32 {
        let [c0, c1, c2] = self.cols;
        c0.dot(c1.cross(c2))
    }

    /// The inverse, or `None` when the determinant is zero or the result
    /// would not be finite.
    #[must_use]
    pub fn inverse(&self) -> Option<Mat3> {
        let [c0, c1, c2] = self.cols;
        // Rows of the inverse are the cross products of column pairs over
        // the determinant (the adjugate formula).
        let r0 = c1.cross(c2);
        let r1 = c2.cross(c0);
        let r2 = c0.cross(c1);
        let det = c0.dot(r0);
        if det == 0.0 || !det.is_finite() {
            return None;
        }
        let inv = 1.0 / det;
        let m = Mat3::from_cols(r0 * inv, r1 * inv, r2 * inv).transpose();
        m.cols.iter().all(|c| c.is_finite()).then_some(m)
    }

    /// `self * v`.
    #[must_use]
    pub fn mul_vec3(&self, v: Vec3) -> Vec3 {
        self.cols[0] * v.x + self.cols[1] * v.y + self.cols[2] * v.z
    }

    /// The matrix for transforming normals: the inverse transpose. `None`
    /// for a singular matrix.
    #[must_use]
    pub fn normal_matrix(&self) -> Option<Mat3> {
        self.inverse().map(|m| m.transpose())
    }
}

impl Mul for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Mat3) -> Mat3 {
        Mat3::from_cols(
            self.mul_vec3(rhs.cols[0]),
            self.mul_vec3(rhs.cols[1]),
            self.mul_vec3(rhs.cols[2]),
        )
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Mat3) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3(rhs)
    }
}

// Exact float comparisons below are on values that are exactly
// representable and computed without rounding.
#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn identity_and_transpose() {
        let m = Mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0]);
        assert_eq!(Mat3::IDENTITY * m, m);
        assert_eq!(m * Mat3::IDENTITY, m);
        assert_eq!(m.transpose().transpose(), m);
        assert_eq!(m.row(0), Vec3::new(1.0, 4.0, 7.0));
        assert_eq!(Mat3::from_cols_array(&m.to_cols_array()), m);
        assert_eq!(Mat3::default(), Mat3::IDENTITY);
    }

    #[test]
    fn determinant_and_inverse() {
        let m = Mat3::from_cols_array(&[2.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 8.0]);
        assert_eq!(m.determinant(), 64.0);
        let inv = m.inverse().unwrap();
        assert_eq!(inv, Mat3::from_scale(Vec3::new(0.5, 0.25, 0.125)));
        assert_eq!(Mat3::ZERO.inverse(), None);
        let singular = Mat3::from_cols(Vec3::X, Vec3::X, Vec3::Z);
        assert_eq!(singular.inverse(), None);
    }

    #[test]
    fn rotation_about_z_turns_x_into_y() {
        let m = Mat3::from_axis_angle(Vec3::Z, core::f32::consts::FRAC_PI_2);
        let v = m * Vec3::X;
        assert!((v - Vec3::Y).length() < 1e-6);
        // A pure rotation's normal matrix is itself.
        let n = m.normal_matrix().unwrap();
        for (a, b) in n.to_cols_array().iter().zip(m.to_cols_array()) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
