//! [`Mat4`]: a 4x4 column-major matrix (transforms, projections).

use core::ops::{Mul, MulAssign};

use crate::{Mat3, Quat, Vec3, Vec4};

/// A 4x4 matrix of four [`Vec4`] columns. See the crate docs for the
/// conventions: column vectors (`m * v`), column-major storage matching a
/// GLSL `mat4`, translation in column 3, and how Direct3D 9 matrices map
/// onto it without a transpose.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    /// Columns 0 to 3.
    pub cols: [Vec4; 4],
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::IDENTITY
    }
}

impl Mat4 {
    /// The identity.
    pub const IDENTITY: Mat4 = Mat4::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);
    /// All zero.
    pub const ZERO: Mat4 = Mat4::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);
    /// Negates clip-space y: turns a y-up (Direct3D 9 style) projection into
    /// a y-down Vulkan one when applied after it (`FLIP_Y * projection`).
    pub const FLIP_Y: Mat4 =
        Mat4::from_cols(Vec4::X, Vec4::new(0.0, -1.0, 0.0, 0.0), Vec4::Z, Vec4::W);

    /// A matrix from its columns.
    #[must_use]
    pub const fn from_cols(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Mat4 {
        Mat4 {
            cols: [c0, c1, c2, c3],
        }
    }

    /// A matrix from sixteen floats in column-major order (the memory order
    /// of a GLSL `mat4`).
    #[must_use]
    pub const fn from_cols_array(a: &[f32; 16]) -> Mat4 {
        Mat4::from_cols(
            Vec4::new(a[0], a[1], a[2], a[3]),
            Vec4::new(a[4], a[5], a[6], a[7]),
            Vec4::new(a[8], a[9], a[10], a[11]),
            Vec4::new(a[12], a[13], a[14], a[15]),
        )
    }

    /// The sixteen floats in column-major order.
    #[must_use]
    pub const fn to_cols_array(&self) -> [f32; 16] {
        let [c0, c1, c2, c3] = self.cols;
        [
            c0.x, c0.y, c0.z, c0.w, c1.x, c1.y, c1.z, c1.w, c2.x, c2.y, c2.z, c2.w, c3.x, c3.y,
            c3.z, c3.w,
        ]
    }

    /// A matrix from sixteen floats written row by row, as in a textbook:
    /// `a[0..4]` is the first row. Use this for hand-written literals.
    #[must_use]
    pub fn from_rows_array(a: &[f32; 16]) -> Mat4 {
        Mat4::from_cols_array(a).transpose()
    }

    /// A Direct3D 9 matrix (row vectors, row-major storage) as the same
    /// transform under this crate's convention. The floats are taken in
    /// memory order with no transpose; see the crate docs for why that is
    /// correct. Composition order flips: Direct3D `A * B` is `B * A` here.
    #[must_use]
    pub const fn from_d3d_row_major(a: &[f32; 16]) -> Mat4 {
        Mat4::from_cols_array(a)
    }

    /// The inverse of [`Mat4::from_d3d_row_major`]: the sixteen floats a
    /// Direct3D 9 matrix with the same meaning would store.
    #[must_use]
    pub const fn to_d3d_row_major(&self) -> [f32; 16] {
        self.to_cols_array()
    }

    /// Row `i` as a vector.
    ///
    /// # Panics
    ///
    /// Panics when `i > 3`.
    #[must_use]
    pub fn row(&self, i: usize) -> Vec4 {
        Vec4::new(
            self.cols[0][i],
            self.cols[1][i],
            self.cols[2][i],
            self.cols[3][i],
        )
    }

    /// The transpose.
    #[must_use]
    pub fn transpose(&self) -> Mat4 {
        Mat4::from_cols(self.row(0), self.row(1), self.row(2), self.row(3))
    }

    /// A translation.
    #[must_use]
    pub const fn from_translation(t: Vec3) -> Mat4 {
        Mat4::from_cols(Vec4::X, Vec4::Y, Vec4::Z, t.extend(1.0))
    }

    /// A non-uniform scale.
    #[must_use]
    pub const fn from_scale(s: Vec3) -> Mat4 {
        Mat4::from_cols(
            Vec4::new(s.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, s.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, s.z, 0.0),
            Vec4::W,
        )
    }

    /// A linear transform from a [`Mat3`], with no translation.
    #[must_use]
    pub const fn from_mat3(m: &Mat3) -> Mat4 {
        Mat4::from_cols(
            m.cols[0].extend(0.0),
            m.cols[1].extend(0.0),
            m.cols[2].extend(0.0),
            Vec4::W,
        )
    }

    /// The rotation of a unit quaternion.
    #[must_use]
    pub fn from_quat(q: Quat) -> Mat4 {
        Mat4::from_mat3(&Mat3::from_quat(q))
    }

    /// A rotation of `angle` radians about a unit `axis`.
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Mat4 {
        Mat4::from_quat(Quat::from_axis_angle(axis, angle))
    }

    /// Scale, then rotate, then translate: `T * R * S`. The usual object
    /// transform; [`Mat4::to_scale_rotation_translation`] undoes it.
    #[must_use]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Mat4 {
        let r = Mat3::from_quat(rotation);
        Mat4::from_cols(
            (r.cols[0] * scale.x).extend(0.0),
            (r.cols[1] * scale.y).extend(0.0),
            (r.cols[2] * scale.z).extend(0.0),
            translation.extend(1.0),
        )
    }

    /// The translation part (column 3).
    #[must_use]
    pub const fn translation(&self) -> Vec3 {
        self.cols[3].truncate()
    }

    /// The determinant.
    #[must_use]
    pub fn determinant(&self) -> f32 {
        let [a, b, c, d] = self.cols;
        // Expansion by 2x2 minors of the top two and bottom two rows.
        let s0 = a.x * b.y - b.x * a.y;
        let s1 = a.x * c.y - c.x * a.y;
        let s2 = a.x * d.y - d.x * a.y;
        let s3 = b.x * c.y - c.x * b.y;
        let s4 = b.x * d.y - d.x * b.y;
        let s5 = c.x * d.y - d.x * c.y;
        let c5 = c.z * d.w - d.z * c.w;
        let c4 = b.z * d.w - d.z * b.w;
        let c3 = b.z * c.w - c.z * b.w;
        let c2 = a.z * d.w - d.z * a.w;
        let c1 = a.z * c.w - c.z * a.w;
        let c0 = a.z * b.w - b.z * a.w;
        s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0
    }

    /// The general inverse, or `None` when the determinant is zero or the
    /// result would not be finite.
    #[must_use]
    // a, b, c, d are the four columns, named as in the textbook formula.
    #[allow(clippy::many_single_char_names)]
    pub fn inverse(&self) -> Option<Mat4> {
        let [a, b, c, d] = self.cols;
        // The same 2x2 minors as `determinant`, reused for the adjugate.
        let s0 = a.x * b.y - b.x * a.y;
        let s1 = a.x * c.y - c.x * a.y;
        let s2 = a.x * d.y - d.x * a.y;
        let s3 = b.x * c.y - c.x * b.y;
        let s4 = b.x * d.y - d.x * b.y;
        let s5 = c.x * d.y - d.x * c.y;
        let c5 = c.z * d.w - d.z * c.w;
        let c4 = b.z * d.w - d.z * b.w;
        let c3 = b.z * c.w - c.z * b.w;
        let c2 = a.z * d.w - d.z * a.w;
        let c1 = a.z * c.w - c.z * a.w;
        let c0 = a.z * b.w - b.z * a.w;
        let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;
        if det == 0.0 || !det.is_finite() {
            return None;
        }
        let inv = 1.0 / det;
        // Columns of the adjugate; each is divided by the determinant.
        let r0 = Vec4::new(
            b.y * c5 - c.y * c4 + d.y * c3,
            -a.y * c5 + c.y * c2 - d.y * c1,
            a.y * c4 - b.y * c2 + d.y * c0,
            -a.y * c3 + b.y * c1 - c.y * c0,
        );
        let r1 = Vec4::new(
            -b.x * c5 + c.x * c4 - d.x * c3,
            a.x * c5 - c.x * c2 + d.x * c1,
            -a.x * c4 + b.x * c2 - d.x * c0,
            a.x * c3 - b.x * c1 + c.x * c0,
        );
        let r2 = Vec4::new(
            b.w * s5 - c.w * s4 + d.w * s3,
            -a.w * s5 + c.w * s2 - d.w * s1,
            a.w * s4 - b.w * s2 + d.w * s0,
            -a.w * s3 + b.w * s1 - c.w * s0,
        );
        let r3 = Vec4::new(
            -b.z * s5 + c.z * s4 - d.z * s3,
            a.z * s5 - c.z * s2 + d.z * s1,
            -a.z * s4 + b.z * s2 - d.z * s0,
            a.z * s3 - b.z * s1 + c.z * s0,
        );
        let m = Mat4::from_cols(r0 * inv, r1 * inv, r2 * inv, r3 * inv);
        m.cols.iter().all(|col| col.is_finite()).then_some(m)
    }

    /// `self * v`.
    #[must_use]
    pub fn mul_vec4(&self, v: Vec4) -> Vec4 {
        self.cols[0] * v.x + self.cols[1] * v.y + self.cols[2] * v.z + self.cols[3] * v.w
    }

    /// Transforms a point (`w = 1`) and drops `w` without dividing: correct
    /// for affine transforms (object and view matrices).
    #[must_use]
    pub fn transform_point3(&self, p: Vec3) -> Vec3 {
        self.mul_vec4(p.extend(1.0)).truncate()
    }

    /// Transforms a point (`w = 1`) and divides by the resulting `w`: use
    /// for projections. A point on the camera plane gives non-finite output.
    #[must_use]
    pub fn project_point3(&self, p: Vec3) -> Vec3 {
        let h = self.mul_vec4(p.extend(1.0));
        h.truncate() / h.w
    }

    /// Transforms a direction (`w = 0`): translation does not apply.
    #[must_use]
    pub fn transform_vector3(&self, v: Vec3) -> Vec3 {
        self.mul_vec4(v.extend(0.0)).truncate()
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4::from_cols(
            self.mul_vec4(rhs.cols[0]),
            self.mul_vec4(rhs.cols[1]),
            self.mul_vec4(rhs.cols[2]),
            self.mul_vec4(rhs.cols[3]),
        )
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs;
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.mul_vec4(rhs)
    }
}

// Exact float comparisons below are on values that are exactly
// representable and computed without rounding.
#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    fn sample() -> Mat4 {
        Mat4::from_rows_array(&[
            2.0, 0.0, 1.0, 3.0, //
            1.0, 3.0, 0.0, -1.0, //
            0.0, 1.0, 4.0, 2.0, //
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[test]
    fn rows_and_columns() {
        let m = sample();
        assert_eq!(m.row(0), Vec4::new(2.0, 0.0, 1.0, 3.0));
        assert_eq!(m.cols[3], Vec4::new(3.0, -1.0, 2.0, 1.0));
        assert_eq!(m.translation(), Vec3::new(3.0, -1.0, 2.0));
        assert_eq!(Mat4::from_cols_array(&m.to_cols_array()), m);
        assert_eq!(m.transpose().transpose(), m);
        assert_eq!(Mat4::default(), Mat4::IDENTITY);
    }

    #[test]
    fn determinant_matches_hand_expansion() {
        // Upper-left 3x3 of `sample` has determinant
        // 2*(3*4 - 0*1) - 0 + 1*(1*1 - 3*0) = 25; the last row is (0,0,0,1).
        assert_eq!(sample().determinant(), 25.0);
        assert_eq!(Mat4::IDENTITY.determinant(), 1.0);
        assert_eq!(
            Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0)).determinant(),
            24.0
        );
    }

    #[test]
    fn inverse_of_simple_transforms() {
        let t = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(
            t.inverse().unwrap(),
            Mat4::from_translation(Vec3::new(-1.0, -2.0, -3.0))
        );
        let s = Mat4::from_scale(Vec3::new(2.0, 4.0, 8.0));
        assert_eq!(
            s.inverse().unwrap(),
            Mat4::from_scale(Vec3::new(0.5, 0.25, 0.125))
        );
        assert_eq!(Mat4::ZERO.inverse(), None);
        let m = sample();
        let p = m.inverse().unwrap() * m;
        for (a, b) in p.to_cols_array().iter().zip(Mat4::IDENTITY.to_cols_array()) {
            assert!((a - b).abs() < 1e-6, "{p:?}");
        }
    }

    #[test]
    fn points_vectors_and_projection() {
        let t = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.transform_point3(Vec3::ZERO), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.transform_vector3(Vec3::X), Vec3::X);
        let mut h = Mat4::IDENTITY;
        h.cols[3].w = 2.0;
        // The point gets w = 1 * 2 from column 3, so it is halved.
        assert_eq!(
            h.project_point3(Vec3::new(2.0, 4.0, 6.0)),
            Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn d3d_row_major_needs_no_transpose() {
        // A Direct3D 9 translation by (5, 6, 7): identity rows with the
        // offset in the fourth row, i.e. memory elements 12, 13 and 14.
        let d3d = [
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            5.0, 6.0, 7.0, 1.0,
        ];
        let m = Mat4::from_d3d_row_major(&d3d);
        assert_eq!(m.transform_point3(Vec3::ZERO), Vec3::new(5.0, 6.0, 7.0));
        assert_eq!(m.to_d3d_row_major(), d3d);
        // Direct3D `A * B` applies A first; here that is `B * A`. Rows of a
        // row-vector product are computed explicitly to check.
        let a = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)).to_d3d_row_major();
        let b = Mat4::from_scale(Vec3::splat(2.0)).to_d3d_row_major();
        let mut ab = [0.0f32; 16];
        for r in 0..4 {
            for k in 0..4 {
                ab[r * 4 + k] = (0..4).map(|j| a[r * 4 + j] * b[j * 4 + k]).sum();
            }
        }
        let ours = Mat4::from_d3d_row_major(&b) * Mat4::from_d3d_row_major(&a);
        assert_eq!(Mat4::from_d3d_row_major(&ab), ours);
        // Translate by 1 then scale by 2: the origin lands on x = 2.
        assert_eq!(ours.transform_point3(Vec3::ZERO), Vec3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn flip_y_negates_clip_y_only() {
        let v = Mat4::FLIP_Y * Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v, Vec4::new(1.0, -2.0, 3.0, 4.0));
    }
}
