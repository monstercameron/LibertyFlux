//! Lifted montage container: the replay editor's clip-list panel, as ordinary Rust.
//!
//! The original's montage container is a user-interface class with eighteen
//! named methods. Three methods with verified rewrites are lifted here: the
//! state dispatcher, the per-item visibility commit, and the indexed pick
//! with its optional epilogue. The other fifteen methods are not lifted (see
//! [`registry`]); fourteen do not yet have verified rewrites, and the long
//! update routine remains unlifted.
//!
//! [`MontageContainer`] owns the object's words as named fields. Its
//! collaborators travel as opaque [`Handle32`] cookies. Dispatcher calls
//! use [`MontageWorld`]; item geometry, registry and listener calls use
//! [`ItemWorld`]. Shared state is supplied through [`MontageGlobals`] and
//! [`ItemGlobals`]. Nothing here has a native address, virtual slot number,
//! pointer-width dependency or `unsafe`.
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
mod indexed_pick;
mod item_commit;
pub mod item_world;
mod world;

pub mod registry;

pub use container::{MontageContainer, MontageGlobals};
pub use item_world::{
    GeometryObject, GeometryValue, ItemGlobals, ItemTag, ItemWorld, ListenerTag, NotifyContextTag,
    RegistryTag, TailContextTag, Viewport,
};
pub use world::{
    ActiveTag, AuxTag, CfgSubTag, CfgTag, ElemTag, EpilogueTag, HelperContextTag, InnerTag,
    ListTag, LookupContextTag, MontageWorld, OutTag, OutputTag, OwnTag, ProbeTag, SessionTag,
    SrcTag, StateTag, Tag, TailTag, ViewTag,
};
