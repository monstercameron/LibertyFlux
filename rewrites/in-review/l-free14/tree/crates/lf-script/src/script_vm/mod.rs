//! Lifted script-VM native thunks: the `script_vm_*` free-function group.
//!
//! The script virtual machine exposes engine behaviour to scripts through
//! thin native routines: script words arrive as arguments, the routine
//! packs float triples into frame buffers (converting a low altitude
//! through a ground query first), and forwards everything to engine
//! callees. These routines belong to no class; what they share is the
//! calling protocol, so each sub-group below owns the piece of state its
//! routines share and each verified routine with behaviour in it is
//! restated as a method on it.
//!
//! [`AltitudeGate`] owns the altitude threshold word behind the eight
//! restart/area routines: one conversion head with five tail shapes.
//! [`SkipFlags`] carries the per-instance flags of the three pack-and-skip
//! routines, all proved against one generic method. [`AreaProbe`] is the
//! stateless bullet/box/extent test protocol: priming, corner
//! normalisation and single-precision expansion with the original's
//! operand order. Proof is differential: every lifted method runs against
//! its verified rewrite on the same generated inputs, comparing results
//! and every effect (see the `lf-scriptvmdiff` test crate). Nothing here
//! is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per routine in [`registry`].

#![forbid(unsafe_code)]

mod altitude;
mod area;
mod skip;

pub mod registry;

pub use altitude::{AltitudeGate, CONV_MODE, GroundQuery, SinkClear6, SinkReg, SinkReg2, SinkTail4};
pub use area::{
    AreaProbe, ExtentRoutine, ExtentTag, Prime, TestBox, TestPoint, TestViews, EXTENT_ARG_A,
    EXTENT_ARG_B,
};
pub use skip::{PointSkip, SkipFlags, SkipSink};
