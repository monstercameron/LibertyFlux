//! Lifted classes of the motion composer: the combining filter of the
//! composer's poll tree, the optimised data block it is built with, and the
//! composer's own sequence, forwarding, table, gate and drain methods (the
//! composer's pending list and pool are modelled only where these methods
//! touch them).
//!
//! Each type owns its data as ordinary Rust: no addresses, no numbered
//! callee slots, no virtual tables, no global state, no pointer width and
//! no `unsafe`. Each verified method with behaviour in it is restated as a
//! method; calls the original makes to code this lane does not lift are
//! methods of a trait the caller implements, and the differential test
//! compares every such call in order with its arguments. Operating-system
//! and thread state (the thread-local block the original reads its owner and
//! manager from) enters as an argument, never through a global.
//!
//! Proof is differential: the `lf-composerdiff` test crate runs each lifted
//! method and its verified 32-bit rewrite on the same generated inputs, on
//! the 32-bit target, comparing results, every call and every written word,
//! floats bit for bit. What each proof leaves out is recorded per method in
//! [`registry`]. Nothing here is verified by the checker itself.

#![forbid(unsafe_code)]

pub mod combining_filter;
pub mod optimized;
pub mod optimized_data;
pub mod registry;

pub use combining_filter::{
    BaseFilterCallees, CombiningFilter, FilterOwner, PollChild,
};
pub use optimized_data::{
    BlockId, ComposerDataCallees, ComposerOptimizedData, ObjectManager, TableEntry, ValueEntry,
};
pub use optimized::{
    ComposerCallees, ComposerId, ComposerOptimized, HandlerId, InnerId, ItemStack,
    NodeId, NodeLinks, OutId, OutWord, PoolNode, TableHandler,
};
