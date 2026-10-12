//! Lifted animation channel decoders.
//!
//! The original keeps one small object per animated track (a float, an
//! integer, a boolean, a vector or a quaternion evolving over frames) and
//! picks the codec per track: constant ([`StaticFloat`], [`StaticInt`],
//! [`StaticVec3`], [`StaticQuat`]), uncompressed keys ([`RawFloat`],
//! [`RawInt`], [`RawBool`], [`RawVec3`], [`RawQuat`]), polynomial segments
//! ([`CurveFloat`]), or compressed forms of which only the size is lifted
//! yet ([`QuantizeFloat`], [`DeltaFloat`], [`RleInt`]).
//!
//! Each type owns its keys as ordinary Rust data (scalars and vectors: no
//! addresses, no virtual tables, no allocator calls) and each verified
//! 32-bit method with behaviour in it is restated as a method on it.
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect, floats bit for bit (see the `lf-animchan-diff` test crate).
//! Nothing here is verified by the checker itself.
//!
//! How this relates to the format reader (`lf-anim`): the reader parses
//! channel objects out of animation files (a constant quaternion, packed
//! per-frame sample words, or opaque headers for codecs it does not
//! decode yet). These channel types take what the reader produces once it
//! is decoded: [`StaticQuat::new`] takes the four floats of the reader's
//! constant quaternion, and the raw channels take plain key vectors, one
//! element per frame, in frame order. No layout is defined twice: vector
//! and quaternion samples are [`lf_math::Vec3`], [`lf_math::Vec4`] and
//! [`lf_math::Quat`].
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`StaticQuat::new`]: StaticQuat::new

#![forbid(unsafe_code)]

mod curve_float;
mod delta_float;
pub mod frame;
mod quantize_float;
mod raw_bool;
mod raw_float;
mod raw_int;
mod raw_quat;
mod raw_vec3;
mod rle_int;
mod static_float;
mod static_int;
mod static_quat;
mod static_vec3;

pub mod registry;

pub use curve_float::{CurveFloat, CurveKey};
pub use delta_float::DeltaFloat;
pub use frame::AnimChannel;
pub use quantize_float::QuantizeFloat;
pub use raw_bool::RawBool;
pub use raw_float::RawFloat;
pub use raw_int::RawInt;
pub use raw_quat::RawQuat;
pub use raw_vec3::RawVec3;
pub use rle_int::RleInt;
pub use static_float::StaticFloat;
pub use static_int::StaticInt;
pub use static_quat::StaticQuat;
pub use static_vec3::StaticVec3;
