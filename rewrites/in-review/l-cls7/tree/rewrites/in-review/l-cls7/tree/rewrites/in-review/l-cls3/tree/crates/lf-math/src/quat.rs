//! [`Quat`]: rotation quaternions.

use core::ops::{Mul, MulAssign, Neg};

use crate::{Mat3, Vec3, Vec4};

/// Below this `|dot|` gap from 1, [`Quat::slerp`] falls back to normalised
/// linear interpolation: the angle is too small for `sin` to divide by.
const SLERP_LINEAR_THRESHOLD: f32 = 1.0e-4;

/// A quaternion `x i + y j + z k + w`. Unit quaternions are rotations; see
/// the crate docs for the conventions (`q v q*`, `a * b` applies `b` first,
/// `q` and `-q` rotate alike).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    /// `i` component.
    pub x: f32,
    /// `j` component.
    pub y: f32,
    /// `k` component.
    pub z: f32,
    /// Scalar component.
    pub w: f32,
}

impl Default for Quat {
    fn default() -> Self {
        Quat::IDENTITY
    }
}

impl Quat {
    /// No rotation.
    pub const IDENTITY: Quat = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);

    /// A quaternion from its components (not normalised).
    #[must_use]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat { x, y, z, w }
    }

    /// The components as a [`Vec4`] (`x, y, z, w`).
    #[must_use]
    pub const fn to_vec4(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }

    /// A quaternion from a [`Vec4`] (`x, y, z, w`).
    #[must_use]
    pub const fn from_vec4(v: Vec4) -> Quat {
        Quat::from_xyzw(v.x, v.y, v.z, v.w)
    }

    /// A rotation of `angle` radians about a unit `axis`.
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Quat {
        let (s, c) = (angle * 0.5).sin_cos();
        let v = axis * s;
        Quat::from_xyzw(v.x, v.y, v.z, c)
    }

    /// A rotation about X.
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Quat {
        Quat::from_axis_angle(Vec3::X, angle)
    }

    /// A rotation about Y.
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Quat {
        Quat::from_axis_angle(Vec3::Y, angle)
    }

    /// A rotation about Z.
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Quat {
        Quat::from_axis_angle(Vec3::Z, angle)
    }

    /// The rotation of an orthonormal, right-handed rotation matrix
    /// (determinant +1). Other matrices give a meaningless result; scale
    /// must be removed first (see [`crate::Mat4::to_scale_rotation_translation`]).
    #[must_use]
    pub fn from_mat3(m: &Mat3) -> Quat {
        // Element (row r, column c) is cols[c][r]. The branch on the largest
        // diagonal term keeps the square root away from zero.
        let [c0, c1, c2] = m.cols;
        let (m00, m11, m22) = (c0.x, c1.y, c2.z);
        let trace = m00 + m11 + m22;
        let q = if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Quat::from_xyzw(
                (c1.z - c2.y) / s,
                (c2.x - c0.z) / s,
                (c0.y - c1.x) / s,
                0.25 * s,
            )
        } else if m00 > m11 && m00 > m22 {
            let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
            Quat::from_xyzw(
                0.25 * s,
                (c1.x + c0.y) / s,
                (c2.x + c0.z) / s,
                (c1.z - c2.y) / s,
            )
        } else if m11 > m22 {
            let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
            Quat::from_xyzw(
                (c1.x + c0.y) / s,
                0.25 * s,
                (c2.y + c1.z) / s,
                (c2.x - c0.z) / s,
            )
        } else {
            let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
            Quat::from_xyzw(
                (c2.x + c0.z) / s,
                (c2.y + c1.z) / s,
                0.25 * s,
                (c0.y - c1.x) / s,
            )
        };
        q.normalize()
    }

    /// The shortest rotation taking unit vector `from` onto unit vector `to`.
    /// Opposite vectors rotate half a turn about some perpendicular axis.
    #[must_use]
    pub fn from_rotation_arc(from: Vec3, to: Vec3) -> Quat {
        let d = from.dot(to);
        if d < -1.0 + 1.0e-6 {
            return Quat::from_axis_angle(from.any_orthonormal(), core::f32::consts::PI);
        }
        let c = from.cross(to);
        Quat::from_xyzw(c.x, c.y, c.z, 1.0 + d).normalize()
    }

    /// The rotation axis (unit) and angle in `0..=2π` radians. The identity
    /// gives axis X and angle 0.
    #[must_use]
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        let q = self.normalize();
        let v = Vec3::new(q.x, q.y, q.z);
        let s = v.length();
        if s <= f32::EPSILON {
            return (Vec3::X, 0.0);
        }
        (v / s, 2.0 * s.atan2(q.w))
    }

    /// The dot product of the components (the cosine of half the angle
    /// between two unit rotations).
    #[must_use]
    pub fn dot(self, rhs: Quat) -> f32 {
        self.to_vec4().dot(rhs.to_vec4())
    }

    /// The length (1 for a rotation).
    #[must_use]
    pub fn length(self) -> f32 {
        self.to_vec4().length()
    }

    /// Scaled to unit length. A zero quaternion gives non-finite components.
    #[must_use]
    pub fn normalize(self) -> Quat {
        Quat::from_vec4(self.to_vec4().normalize())
    }

    /// True when the length is within `eps` of 1.
    #[must_use]
    pub fn is_normalized(self, eps: f32) -> bool {
        self.to_vec4().is_normalized(eps)
    }

    /// The conjugate: the inverse rotation for a unit quaternion.
    #[must_use]
    pub const fn conjugate(self) -> Quat {
        Quat::from_xyzw(-self.x, -self.y, -self.z, self.w)
    }

    /// The multiplicative inverse (for any non-zero quaternion).
    #[must_use]
    pub fn inverse(self) -> Quat {
        let n = self.dot(self);
        let c = self.conjugate();
        Quat::from_xyzw(c.x / n, c.y / n, c.z / n, c.w / n)
    }

    /// Rotates a vector by a unit quaternion.
    #[must_use]
    pub fn mul_vec3(self, v: Vec3) -> Vec3 {
        // v + 2 u x (u x v + w v), with u the vector part: the expansion of
        // q v q* for unit q.
        let u = Vec3::new(self.x, self.y, self.z);
        let t = u.cross(v) * 2.0;
        v + t * self.w + u.cross(t)
    }

    /// The rotation as a [`Mat3`].
    #[must_use]
    pub fn to_mat3(self) -> Mat3 {
        Mat3::from_quat(self)
    }

    /// Normalised linear interpolation along the shorter arc. Cheaper than
    /// [`Quat::slerp`]; the speed is not constant.
    #[must_use]
    pub fn nlerp(self, end: Quat, t: f32) -> Quat {
        let end = if self.dot(end) < 0.0 { -end } else { end };
        Quat::from_vec4(self.to_vec4().lerp(end.to_vec4(), t)).normalize()
    }

    /// Spherical linear interpolation of unit quaternions along the shorter
    /// arc, at constant angular speed: `self` at `t = 0`, `end` (or `-end`,
    /// the same rotation) at `t = 1`.
    #[must_use]
    pub fn slerp(self, end: Quat, t: f32) -> Quat {
        let mut cos = self.dot(end);
        let mut end = end;
        if cos < 0.0 {
            cos = -cos;
            end = -end;
        }
        if cos > 1.0 - SLERP_LINEAR_THRESHOLD {
            return self.nlerp(end, t);
        }
        let angle = cos.acos();
        let inv_sin = 1.0 / angle.sin();
        let a = ((1.0 - t) * angle).sin() * inv_sin;
        let b = (t * angle).sin() * inv_sin;
        Quat::from_vec4(self.to_vec4() * a + end.to_vec4() * b)
    }

    /// The angle in radians (`0..=π`) of the rotation between two unit
    /// quaternions.
    #[must_use]
    pub fn angle_between(self, rhs: Quat) -> f32 {
        2.0 * self.dot(rhs).abs().min(1.0).acos()
    }
}

impl Mul for Quat {
    type Output = Quat;
    /// The Hamilton product: `self * rhs` applies `rhs` first.
    fn mul(self, rhs: Quat) -> Quat {
        let (a, b) = (self, rhs);
        Quat::from_xyzw(
            a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
            a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
            a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
            a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
        )
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, rhs: Quat) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3(rhs)
    }
}

impl Neg for Quat {
    type Output = Quat;
    fn neg(self) -> Quat {
        Quat::from_xyzw(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::{FRAC_PI_2, PI};

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-5
    }

    #[test]
    fn quarter_turns_follow_the_right_hand_rule() {
        assert!(close(Quat::from_rotation_z(FRAC_PI_2) * Vec3::X, Vec3::Y));
        assert!(close(Quat::from_rotation_x(FRAC_PI_2) * Vec3::Y, Vec3::Z));
        assert!(close(Quat::from_rotation_y(FRAC_PI_2) * Vec3::Z, Vec3::X));
    }

    #[test]
    fn product_applies_right_operand_first() {
        let a = Quat::from_rotation_z(FRAC_PI_2);
        let b = Quat::from_rotation_x(FRAC_PI_2);
        // b first: Y -> Z; then a: Z stays Z.
        assert!(close((a * b) * Vec3::Y, Vec3::Z));
        // a first: Y -> -X; then b: -X stays -X.
        assert!(close((b * a) * Vec3::Y, -Vec3::X));
    }

    #[test]
    fn axis_angle_round_trip() {
        let axis = Vec3::new(1.0, 2.0, 2.0) / 3.0;
        let (got_axis, got_angle) = Quat::from_axis_angle(axis, 1.25).to_axis_angle();
        assert!(close(got_axis, axis));
        assert!((got_angle - 1.25).abs() < 1e-5);
        assert_eq!(Quat::IDENTITY.to_axis_angle(), (Vec3::X, 0.0));
        assert_eq!(Quat::default(), Quat::IDENTITY);
    }

    #[test]
    fn arc_and_inverse() {
        let q = Quat::from_rotation_arc(Vec3::X, Vec3::Y);
        assert!(close(q * Vec3::X, Vec3::Y));
        let half = Quat::from_rotation_arc(Vec3::Z, -Vec3::Z);
        assert!(close(half * Vec3::Z, -Vec3::Z));
        let r = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
        let p = r * r.inverse();
        assert!((p.w - 1.0).abs() < 1e-6 && Vec3::new(p.x, p.y, p.z).length() < 1e-6);
        assert!((Quat::from_rotation_x(PI).angle_between(Quat::IDENTITY) - PI).abs() < 1e-5);
    }

    #[test]
    fn slerp_halfway_is_half_the_angle() {
        let a = Quat::IDENTITY;
        let b = Quat::from_rotation_z(FRAC_PI_2);
        let mid = a.slerp(b, 0.5);
        assert!(close(mid * Vec3::X, Vec3::new(1.0, 1.0, 0.0).normalize()));
        // Nearly equal inputs take the linear path and stay unit length.
        let near = Quat::from_rotation_z(1e-5);
        assert!(a.slerp(near, 0.5).is_normalized(1e-6));
    }
}
