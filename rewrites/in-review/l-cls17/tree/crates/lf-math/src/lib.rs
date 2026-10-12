//! `lf-math`: shared maths types for the renderer and the simulation.
//!
//! README for future lanes:
//! - [`Vec2`], [`Vec3`], [`Vec4`], [`Mat3`], [`Mat4`] and [`Quat`] are plain
//!   `f32` types with a fixed `#[repr(C)]` layout, no dependencies and no
//!   `unsafe`. Every engine crate uses these; nothing invents its own vector
//!   or matrix type. (`lf-core`'s README still says maths will live there
//!   once a maths crate is approved; this crate is that home, written
//!   in-tree instead of downloaded.)
//! - [`approx`] holds the tolerance comparisons tests and gameplay code use
//!   instead of `==` on floats.
//! - The conventions below are a specification. Code that converts to or
//!   from another convention (the game's Direct3D 9 data, Vulkan clip space,
//!   glTF) does it at the boundary with the helpers named here.
//!
//! # Conventions
//!
//! **Column vectors, column-major storage.** A matrix transforms a vector
//! as `m * v`. `a * b` applies `b` first, then `a`. [`Mat4`] stores four
//! [`Vec4`] columns, so its sixteen floats in memory are column 0, then
//! column 1, and so on: exactly what a GLSL `mat4` in a Vulkan uniform or
//! push-constant block expects by default. Translation lives in column 3
//! (memory elements 12, 13 and 14).
//!
//! **The game's Direct3D 9 matrices need no transpose.** Direct3D 9 (and
//! D3DX) uses row vectors (`v * M`) with row-major storage, translation in
//! the fourth row, which is also memory elements 12, 13 and 14. Transposing
//! the convention and transposing the storage cancel out: the sixteen floats
//! of a Direct3D 9 matrix, read in memory order as columns, are the same
//! transform under this crate's convention. Use
//! [`Mat4::from_d3d_row_major`] (a documented alias of
//! [`Mat4::from_cols_array`]) so the intent is visible. Composition order
//! does flip: the Direct3D product `A * B` (A first) is `B * A` here.
//!
//! **Handedness.** All helpers here are right-handed. The game's world space
//! is taken to be right-handed with +Z up (x east, y north), which is what
//! the model and map data look like (Inferred; to be confirmed against the
//! running game, a local check). View space is right-handed with the camera
//! at the origin looking down -Z, +Y up and +X right.
//!
//! **Clip space is Vulkan's.** The projection helpers ([`Mat4::perspective_vk`]
//! and friends) map view space to Vulkan clip space: x right, **y down**
//! (normalised device y = -1 is the top of the screen), depth **0 at the near
//! plane and 1 at the far plane** (or reversed, for the reverse-Z variants),
//! and `w = -z_view`, positive in front of the camera. Direct3D 9 clip space
//! shares the 0..1 depth range but has y up; a Direct3D-style projection
//! becomes a Vulkan one by negating clip y, `Mat4::FLIP_Y * projection`.
//! Which projection the game itself builds (left- or right-handed, which
//! field of view) is Unknown until it is read from the running game.
//! Flipping y also flips the apparent winding of triangles: a renderer that
//! uses `FLIP_Y` (or a negative viewport height) swaps its front-face
//! setting.
//!
//! **Angles are radians.** Positive rotation is counter-clockwise when
//! looking from the positive end of the axis towards the origin (the
//! right-hand rule).
//!
//! **Quaternions** are stored `x, y, z, w` with `w` the scalar part; a unit
//! quaternion `q` rotates `v` as `q v q*`, and `a * b` applies `b` first, as
//! with matrices. `q` and `-q` are the same rotation.
//!
//! **GPU layout notes.** [`Vec3`] is 12 bytes and [`Mat3`] 36 bytes (three
//! tightly packed columns). Vulkan's std140 and std430 rules align a `vec3`
//! to 16 bytes and pad every `mat3` column to 16 bytes, so upload a [`Mat3`]
//! as a [`Mat4`] (or pad it) and place [`Vec3`] fields with care.

pub mod approx;
mod mat3;
mod mat4;
mod quat;
mod transform;
mod vec;

pub use approx::ApproxEq;
pub use mat3::Mat3;
pub use mat4::Mat4;
pub use quat::Quat;
pub use vec::{Vec2, Vec3, Vec4};

#[cfg(test)]
mod layout_tests {
    use super::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
    use core::mem::{offset_of, size_of};

    #[test]
    fn layouts_are_tightly_packed_f32() {
        assert_eq!(size_of::<Vec2>(), 8);
        assert_eq!(size_of::<Vec3>(), 12);
        assert_eq!(size_of::<Vec4>(), 16);
        assert_eq!(size_of::<Quat>(), 16);
        assert_eq!(size_of::<Mat3>(), 36);
        assert_eq!(size_of::<Mat4>(), 64);
        assert_eq!(offset_of!(Vec4, w), 12);
        assert_eq!(offset_of!(Quat, w), 12);
        assert_eq!(offset_of!(Mat4, cols), 0);
    }
}
