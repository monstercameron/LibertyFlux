//! Lifted montage container: the replay editor's clip-list panel, as ordinary Rust.
//!
//! The original's montage container is a user-interface class with eighteen
//! named methods. One of them is lifted here: the state dispatcher, which
//! reads the editor's state flag and runs one of three regions (a list
//! walk that notifies two helpers per clip, a threshold-gated refresh
//! sequence, or a pending-index reconciliation), falling through to the
//! last lookup answer when no identity gate matches. The other seventeen
//! methods are not lifted (see [`registry`]).
//!
//! [`MontageContainer`] owns the object's words as named fields. Its
//! collaborators travel as opaque [`Handle32`] cookies, and every call to
//! them is one method of [`MontageWorld`]. The shared state flag the
//! dispatcher reads is passed in as [`MontageGlobals`]. Nothing here has
//! an address, a virtual slot number, a pointer width or `unsafe`.
//!
//! Proof is differential: the lifted method runs against its verified
//! 32-bit rewrite on the same generated inputs, comparing the result,
//! every written word of the object, and every collaborator call in order
//! with its arguments (see the `lf-montage-diff` test crate). Nothing here
//! is verified by the checker itself.
//!
//! [`Handle32`]: lf_core::Handle32

#![forbid(unsafe_code)]

mod container;
mod world;

pub mod registry;

pub use container::{MontageContainer, MontageGlobals};
pub use world::{
    AuxTag, CfgSubTag, CfgTag, ElemTag, InnerTag, ListTag, MontageWorld, OutTag, OutputTag, OwnTag,
    ProbeTag, SessionTag, SrcTag, Tag, TailTag, ViewTag,
};
