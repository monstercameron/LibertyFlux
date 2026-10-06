//! Camera and object transform helpers on [`Mat4`]: look-at views,
//! Vulkan projections and decomposition.
//!
//! Every helper follows the crate conventions: right-handed view space
//! (camera looking down -Z, +Y up) mapped to Vulkan clip space (y down,
//! depth 0..1, or 1..0 for the reverse-Z variants).

use crate::{Mat3, Mat4, Quat, Vec3, Vec4};

impl Mat4 {
    /// A right-handed view matrix for a camera at `eye` looking towards
    /// `target`, with `up` pointing roughly up (it need not be
    /// perpendicular, but must not be parallel to the view direction).
    ///
    /// The result maps `eye` to the origin and `target` onto the -Z axis,
    /// with `up` projected onto +Y.
    #[must_use]
    pub fn look_at_rh(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        Mat4::look_to_rh(eye, target - eye, up)
    }

    /// Like [`Mat4::look_at_rh`], with a view direction instead of a target.
    #[must_use]
    pub fn look_to_rh(eye: Vec3, dir: Vec3, up: Vec3) -> Mat4 {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Mat4::from_cols(
            Vec4::new(s.x, u.x, -f.x, 0.0),
            Vec4::new(s.y, u.y, -f.y, 0.0),
            Vec4::new(s.z, u.z, -f.z, 0.0),
            Vec4::new(-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0),
        )
    }

    /// A perspective projection to Vulkan clip space: vertical field of view
    /// `fov_y` in radians, `aspect` = width / height, depth 0 at `near` and 1
    /// at `far` (both positive distances, `near < far`). View-space +Y comes
    /// out as negative clip y (the top of the screen).
    #[must_use]
    pub fn perspective_vk(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let f = 1.0 / (0.5 * fov_y).tan();
        let range = 1.0 / (near - far);
        Mat4::from_cols(
            Vec4::new(f / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far * range, -1.0),
            Vec4::new(0.0, 0.0, near * far * range, 0.0),
        )
    }

    /// [`Mat4::perspective_vk`] with reversed depth: 1 at `near`, 0 at `far`.
    /// Reverse Z spreads float depth precision evenly over distance; use it
    /// with a `GREATER` depth test and a depth clear of 0.
    #[must_use]
    pub fn perspective_reverse_z_vk(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let f = 1.0 / (0.5 * fov_y).tan();
        let range = 1.0 / (far - near);
        Mat4::from_cols(
            Vec4::new(f / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, near * range, -1.0),
            Vec4::new(0.0, 0.0, near * far * range, 0.0),
        )
    }

    /// Reverse-Z perspective with no far plane: depth 1 at `near`, tending
    /// to 0 at infinity.
    #[must_use]
    pub fn perspective_infinite_reverse_z_vk(fov_y: f32, aspect: f32, near: f32) -> Mat4 {
        let f = 1.0 / (0.5 * fov_y).tan();
        Mat4::from_cols(
            Vec4::new(f / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, -1.0),
            Vec4::new(0.0, 0.0, near, 0.0),
        )
    }

    /// An orthographic projection to Vulkan clip space of the view-space box
    /// `left..right`, `bottom..top`, depth `near..far` in front of the camera
    /// (depth 0 at `near`, 1 at `far`; y flipped so `top` is clip y = -1).
    #[must_use]
    pub fn orthographic_vk(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Mat4 {
        let rw = 1.0 / (right - left);
        let rh = 1.0 / (top - bottom);
        let rd = 1.0 / (far - near);
        Mat4::from_cols(
            Vec4::new(2.0 * rw, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -2.0 * rh, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -rd, 0.0),
            Vec4::new(-(right + left) * rw, (top + bottom) * rh, -near * rd, 1.0),
        )
    }

    /// Splits an affine transform built as `T * R * S` back into scale,
    /// rotation and translation (see
    /// [`Mat4::from_scale_rotation_translation`]).
    ///
    /// A transform that mirrors (negative determinant) is reported with a
    /// negative x scale, so the rotation stays a proper rotation. Shear and
    /// projection cannot be represented; for such input the result is
    /// unspecified. A zero scale on any axis gives a non-finite rotation.
    #[must_use]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        let linear = Mat3::from_mat4(self);
        let sign = if linear.determinant() < 0.0 {
            -1.0
        } else {
            1.0
        };
        let scale = Vec3::new(
            linear.cols[0].length() * sign,
            linear.cols[1].length(),
            linear.cols[2].length(),
        );
        let rotation = Mat3::from_cols(
            linear.cols[0] / scale.x,
            linear.cols[1] / scale.y,
            linear.cols[2] / scale.z,
        );
        (scale, Quat::from_mat3(&rotation), self.translation())
    }

    /// The inverse of a rigid transform (rotation and translation only),
    /// computed by transposing the rotation: cheaper and more exact than
    /// [`Mat4::inverse`] for view matrices. Wrong for anything with scale.
    #[must_use]
    pub fn inverse_rigid(&self) -> Mat4 {
        let r = Mat3::from_mat4(self).transpose();
        let t = -(r * self.translation());
        let mut m = Mat4::from_mat3(&r);
        m.cols[3] = t.extend(1.0);
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::FRAC_PI_2;

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-4
    }

    #[test]
    fn look_at_puts_target_on_negative_z() {
        let eye = Vec3::new(10.0, -5.0, 3.0);
        let target = Vec3::new(0.0, 0.0, 1.0);
        let view = Mat4::look_at_rh(eye, target, Vec3::Z);
        assert!(close(view.transform_point3(eye), Vec3::ZERO));
        let t = view.transform_point3(target);
        assert!(t.x.abs() < 1e-4 && t.y.abs() < 1e-4 && t.z < 0.0);
        assert!((t.z + eye.distance(target)).abs() < 1e-4);
        // World up ends up with positive view-space y.
        assert!(view.transform_vector3(Vec3::Z).y > 0.0);
        let back = view.inverse_rigid();
        assert!(close(back.transform_point3(Vec3::ZERO), eye));
    }

    #[test]
    fn perspective_depth_and_y_down() {
        let (near, far) = (0.5, 200.0);
        let p = Mat4::perspective_vk(FRAC_PI_2, 16.0 / 9.0, near, far);
        assert!(p.project_point3(Vec3::new(0.0, 0.0, -near)).z.abs() < 1e-6);
        assert!((p.project_point3(Vec3::new(0.0, 0.0, -far)).z - 1.0).abs() < 1e-5);
        // With a 90 degree field of view, a point at 45 degrees up lands on
        // the top edge: clip y = -1.
        let top = p.project_point3(Vec3::new(0.0, 10.0, -10.0));
        assert!((top.y + 1.0).abs() < 1e-5, "{top:?}");
        let right = p.project_point3(Vec3::new(10.0 * 16.0 / 9.0, 0.0, -10.0));
        assert!((right.x - 1.0).abs() < 1e-5);
        // w is the distance in front of the camera.
        let h = p * Vec4::new(0.0, 0.0, -7.0, 1.0);
        assert!((h.w - 7.0).abs() < 1e-6);
    }

    #[test]
    fn reverse_z_variants() {
        let (near, far) = (0.1, 1000.0);
        let p = Mat4::perspective_reverse_z_vk(1.0, 1.5, near, far);
        assert!((p.project_point3(Vec3::new(0.0, 0.0, -near)).z - 1.0).abs() < 1e-5);
        assert!(p.project_point3(Vec3::new(0.0, 0.0, -far)).z.abs() < 1e-6);
        let inf = Mat4::perspective_infinite_reverse_z_vk(1.0, 1.5, near);
        assert!((inf.project_point3(Vec3::new(0.0, 0.0, -near)).z - 1.0).abs() < 1e-6);
        assert!(inf.project_point3(Vec3::new(0.0, 0.0, -1.0e6)).z < 1e-6);
    }

    #[test]
    fn orthographic_maps_the_box_to_clip_space() {
        let o = Mat4::orthographic_vk(-4.0, 4.0, -2.0, 2.0, 1.0, 11.0);
        assert!(close(
            o.transform_point3(Vec3::new(-4.0, 2.0, -1.0)),
            Vec3::new(-1.0, -1.0, 0.0)
        ));
        assert!(close(
            o.transform_point3(Vec3::new(4.0, -2.0, -11.0)),
            Vec3::new(1.0, 1.0, 1.0)
        ));
    }

    #[test]
    fn decompose_recovers_parts_including_mirroring() {
        let rot = Quat::from_axis_angle(Vec3::new(0.0, 0.6, 0.8), 0.7);
        for scale in [Vec3::new(1.0, 2.0, 3.0), Vec3::new(-2.0, 0.5, 4.0)] {
            let t = Vec3::new(5.0, -6.0, 7.0);
            let m = Mat4::from_scale_rotation_translation(scale, rot, t);
            let (s, r, tr) = m.to_scale_rotation_translation();
            assert!(close(s, scale), "{s:?}");
            assert!(r.angle_between(rot) < 1e-3, "{r:?}");
            assert!(close(tr, t));
        }
    }
}
