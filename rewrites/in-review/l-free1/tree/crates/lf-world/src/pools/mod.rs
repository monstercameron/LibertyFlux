//! Lifted object pools: fixed-size slot stores with flag bytes.
//!
//! The original keeps its pooled objects in flat slot stores described by
//! four words: the entry base, the per-slot flag bytes, the slot count and
//! the entry stride. A slot is dead when its flag byte has bit `0x80` set.
//! One pool hangs its descriptor behind a context global ([`SlotPool`] reads
//! it the same way once loaded); the others keep the four words inline.
//!
//! [`PoolVec`] is the second structure: a count-prefixed buffer of stamped
//! fixed-stride slots, built by one initialiser routine with thirteen
//! verified instances.
//!
//! Each type owns its slots as ordinary Rust data (byte vectors: no
//! addresses, no allocator calls) and each verified 32-bit routine with
//! behaviour in it is restated as a method on it. Creation takes its
//! allocator and initialiser as traits; the notifier and the refresh
//! helper are traits too. Proof is differential: every lifted method runs
//! against its verified rewrite on the same generated inputs, comparing
//! results and every effect (see the `lf-pooldiff` test crate). Nothing
//! here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

mod poolvec;
mod slot;

pub mod registry;

pub use poolvec::{ElemStamp, PoolVec, VecAlloc, VecElemTag};
pub use slot::{
    CtxAlloc, CtxHandle, CtxInit, CtxTag, Evict, Notify, Refresh, SlotPool, Survives,
};
