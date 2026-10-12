//! Property-style tests: each property is checked on a few hundred inputs
//! drawn from a small deterministic generator (no crates, no randomness
//! between runs, so a failure always reproduces with the printed case
//! number).

use core::f32::consts::PI;

use lf_math::{ApproxEq, Mat3, Mat4, Quat, Vec3, Vec4};

/// Cases per property.
const CASES: usize = 500;

/// `SplitMix64`: tiny, well-distributed, and fully reproducible.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0.0..1.0` from the top 24 bits (exact in f32).
    #[allow(clippy::cast_precision_loss)] // 24-bit values are exact in f32
    fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.unit()
    }

    fn vec3(&mut self, lo: f32, hi: f32) -> Vec3 {
        Vec3::new(self.range(lo, hi), self.range(lo, hi), self.range(lo, hi))
    }

    /// A unit vector, rejecting near-zero draws.
    fn unit_vec3(&mut self) -> Vec3 {
        loop {
            if let Some(v) = self.vec3(-1.0, 1.0).try_normalize()
                && v.length() > 0.0
            {
                return v;
            }
        }
    }

    fn rotation(&mut self) -> Quat {
        let axis = self.unit_vec3();
        Quat::from_axis_angle(axis, self.range(-PI, PI))
    }

    /// A scale whose components are bounded away from zero, sign random.
    fn scale(&mut self) -> Vec3 {
        let mut s = self.vec3(0.25, 4.0);
        if self.unit() < 0.3 {
            s.x = -s.x;
        }
        s
    }

    fn trs(&mut self) -> Mat4 {
        let (s, r, t) = (self.scale(), self.rotation(), self.vec3(-100.0, 100.0));
        Mat4::from_scale_rotation_translation(s, r, t)
    }

    fn mat4(&mut self) -> Mat4 {
        let mut a = [0.0f32; 16];
        for x in &mut a {
            *x = self.range(-2.0, 2.0);
        }
        Mat4::from_cols_array(&a)
    }
}

/// Largest absolute difference between two matrices' elements.
fn max_diff(a: &Mat4, b: &Mat4) -> f32 {
    a.to_cols_array()
        .iter()
        .zip(b.to_cols_array())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f32::max)
}

#[test]
fn inverse_times_matrix_is_identity_for_transforms() {
    let mut rng = Rng::new(1);
    for case in 0..CASES {
        let m = rng.trs();
        let inv = m.inverse().expect("TRS with non-zero scale is invertible");
        let err = max_diff(&(inv * m), &Mat4::IDENTITY).max(max_diff(&(m * inv), &Mat4::IDENTITY));
        assert!(err < 1e-4, "case {case}: error {err}");
    }
}

#[test]
fn inverse_times_matrix_is_identity_for_general_matrices() {
    let mut rng = Rng::new(2);
    let mut checked = 0;
    for case in 0..CASES {
        let m = rng.mat4();
        // Skip badly conditioned draws: the property is about the formula,
        // not about float precision near singularity.
        if m.determinant().abs() < 0.5 {
            continue;
        }
        let inv = m.inverse().expect("non-singular");
        let err = max_diff(&(inv * m), &Mat4::IDENTITY);
        assert!(err < 1e-3, "case {case}: error {err}");
        checked += 1;
    }
    assert!(
        checked > CASES / 4,
        "too few well-conditioned draws: {checked}"
    );
}

#[test]
fn mat3_inverse_and_determinant_of_products() {
    let mut rng = Rng::new(3);
    for case in 0..CASES {
        let a = Mat3::from_mat4(&rng.trs());
        let b = Mat3::from_mat4(&rng.trs());
        let inv = a.inverse().expect("invertible");
        assert!((inv * a).abs_diff_eq(&Mat3::IDENTITY, 1e-4), "case {case}");
        let lhs = (a * b).determinant();
        let rhs = a.determinant() * b.determinant();
        assert!(
            lhs.relative_eq(&rhs, 1e-4, 1e-4),
            "case {case}: {lhs} vs {rhs}"
        );
    }
}

#[test]
fn determinant_is_multiplicative_and_transpose_reverses_products() {
    let mut rng = Rng::new(4);
    for case in 0..CASES {
        let a = rng.mat4();
        let b = rng.mat4();
        let lhs = (a * b).determinant();
        let rhs = a.determinant() * b.determinant();
        assert!(
            lhs.relative_eq(&rhs, 1e-3, 1e-3),
            "case {case}: {lhs} vs {rhs}"
        );
        assert_eq!(a.transpose().transpose(), a);
        assert!(
            (a * b)
                .transpose()
                .relative_eq(&(b.transpose() * a.transpose()), 1e-5, 1e-5)
        );
        assert!(
            a.transpose()
                .determinant()
                .relative_eq(&a.determinant(), 1e-4, 1e-4)
        );
    }
}

#[test]
fn normalisation_gives_unit_length() {
    let mut rng = Rng::new(5);
    for case in 0..CASES {
        let v = rng.vec3(-1000.0, 1000.0);
        if let Some(n) = v.try_normalize() {
            assert!(n.is_normalized(1e-5), "case {case}: {n:?}");
            assert!(n.dot(v) > 0.0, "case {case}: direction kept");
        }
        let q = Quat::from_vec4(Vec4::new(
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
        ));
        if q.length() > 1e-3 {
            assert!(q.normalize().is_normalized(1e-5), "case {case}");
        }
    }
}

#[test]
fn cross_product_identities() {
    let mut rng = Rng::new(6);
    for case in 0..CASES {
        let a = rng.vec3(-10.0, 10.0);
        let b = rng.vec3(-10.0, 10.0);
        let c = a.cross(b);
        assert!(
            c.dot(a).abs() < 1e-3 && c.dot(b).abs() < 1e-3,
            "case {case}"
        );
        // Lagrange: |a x b|^2 + (a . b)^2 = |a|^2 |b|^2.
        let lhs = c.length_squared() + a.dot(b) * a.dot(b);
        let rhs = a.length_squared() * b.length_squared();
        assert!(
            lhs.relative_eq(&rhs, 1e-2, 1e-4),
            "case {case}: {lhs} vs {rhs}"
        );
        assert!(b.cross(a).approx_eq(&-c));
    }
}

#[test]
fn quaternion_and_matrix_rotations_agree() {
    let mut rng = Rng::new(7);
    for case in 0..CASES {
        let q = rng.rotation();
        let v = rng.vec3(-10.0, 10.0);
        let by_quat = q * v;
        let by_mat = q.to_mat3() * v;
        let by_mat4 = Mat4::from_quat(q).transform_vector3(v);
        assert!(by_quat.abs_diff_eq(&by_mat, 1e-4), "case {case}");
        assert!(by_quat.abs_diff_eq(&by_mat4, 1e-4), "case {case}");
        assert!(
            (by_quat.length() - v.length()).abs() < 1e-4,
            "case {case}: length kept"
        );
        // Matrix -> quaternion -> matrix is the identity on rotations.
        let back = Quat::from_mat3(&q.to_mat3());
        assert!(
            back.same_rotation(&q, 1e-4),
            "case {case}: {back:?} vs {q:?}"
        );
        // Composition matches between the two representations.
        let p = rng.rotation();
        let composed = (q * p).to_mat3();
        assert!(
            composed.abs_diff_eq(&(q.to_mat3() * p.to_mat3()), 1e-4),
            "case {case}"
        );
        // q * q^-1 is the identity rotation.
        assert!(
            (q * q.inverse()).same_rotation(&Quat::IDENTITY, 1e-5),
            "case {case}"
        );
        assert!(
            (q * q.conjugate()).same_rotation(&Quat::IDENTITY, 1e-5),
            "case {case}"
        );
    }
}

#[test]
fn slerp_endpoints_and_constant_speed() {
    let mut rng = Rng::new(8);
    for case in 0..CASES {
        let a = rng.rotation();
        let b = rng.rotation();
        assert!(
            a.slerp(b, 0.0).same_rotation(&a, 1e-4),
            "case {case}: t = 0"
        );
        assert!(
            a.slerp(b, 1.0).same_rotation(&b, 1e-4),
            "case {case}: t = 1"
        );
        let total = a.angle_between(b);
        for t in [0.25f32, 0.5, 0.75] {
            let m = a.slerp(b, t);
            assert!(m.is_normalized(1e-4), "case {case}: unit at t = {t}");
            if total > 1e-2 {
                let part = a.angle_between(m);
                assert!(
                    (part - t * total).abs() < 2e-3,
                    "case {case}: t = {t}, {part} vs {}",
                    t * total
                );
            }
        }
        assert!(
            a.nlerp(b, 0.0).same_rotation(&a, 1e-4),
            "case {case}: nlerp t = 0"
        );
        assert!(
            a.nlerp(b, 1.0).same_rotation(&b, 1e-4),
            "case {case}: nlerp t = 1"
        );
    }
}

#[test]
fn decomposition_round_trips() {
    let mut rng = Rng::new(9);
    for case in 0..CASES {
        let (s, r, t) = (rng.scale(), rng.rotation(), rng.vec3(-100.0, 100.0));
        let m = Mat4::from_scale_rotation_translation(s, r, t);
        let (s2, r2, t2) = m.to_scale_rotation_translation();
        assert!(s2.abs_diff_eq(&s, 1e-4), "case {case}: {s2:?} vs {s:?}");
        assert!(r2.same_rotation(&r, 1e-3), "case {case}: {r2:?} vs {r:?}");
        assert!(t2.abs_diff_eq(&t, 1e-4), "case {case}");
        let rebuilt = Mat4::from_scale_rotation_translation(s2, r2, t2);
        assert!(max_diff(&rebuilt, &m) < 1e-3, "case {case}");
    }
}

#[test]
fn rigid_inverse_matches_general_inverse() {
    let mut rng = Rng::new(10);
    for case in 0..CASES {
        let m =
            Mat4::from_scale_rotation_translation(Vec3::ONE, rng.rotation(), rng.vec3(-50.0, 50.0));
        let fast = m.inverse_rigid();
        let general = m.inverse().unwrap();
        assert!(max_diff(&fast, &general) < 1e-4, "case {case}");
    }
}

#[test]
fn look_at_and_projection_properties() {
    let mut rng = Rng::new(11);
    for case in 0..CASES {
        let eye = rng.vec3(-100.0, 100.0);
        let target = eye + rng.unit_vec3() * rng.range(1.0, 50.0);
        // Pick an up vector not parallel to the view direction.
        let dir = (target - eye).normalize();
        let up = if dir.z.abs() > 0.99 { Vec3::Y } else { Vec3::Z };
        let view = Mat4::look_at_rh(eye, target, up);
        assert!(
            view.transform_point3(eye).abs_diff_eq(&Vec3::ZERO, 1e-3),
            "case {case}"
        );
        let t = view.transform_point3(target);
        assert!(
            t.x.abs() < 1e-3 && t.y.abs() < 1e-3 && t.z < 0.0,
            "case {case}: {t:?}"
        );
        // A view matrix is rigid: determinant 1.
        assert!((view.determinant() - 1.0).abs() < 1e-4, "case {case}");

        let near = rng.range(0.05, 1.0);
        let far = near + rng.range(10.0, 1000.0);
        let proj = Mat4::perspective_vk(rng.range(0.5, 2.0), rng.range(0.5, 3.0), near, far);
        let depth = rng.range(near, far);
        let z = proj.project_point3(Vec3::new(0.0, 0.0, -depth)).z;
        assert!((-1e-5..=1.0 + 1e-5).contains(&z), "case {case}: depth {z}");
        // Depth grows with distance.
        let farther = proj
            .project_point3(Vec3::new(0.0, 0.0, -(depth + 1.0).min(far)))
            .z;
        assert!(farther >= z - 1e-6, "case {case}");
        // Up in view space is negative clip y (Vulkan's y-down).
        assert!(
            proj.project_point3(Vec3::new(0.0, 0.1, -depth)).y < 0.0,
            "case {case}"
        );
    }
}
