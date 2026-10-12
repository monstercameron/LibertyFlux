//! The layout frame of the front end's interface toolkit.
//!
//! [`UILayoutFrame`] owns the bytes of the frame's state that the verified
//! methods touch, and restates the methods of its interface that the lift has
//! reached. Methods that call no other code are on the frame itself. Methods
//! that call out take a host trait for their callees and collaborators:
//! [`follow_up`] (token resolution and follow-up steps), [`blend`] (a clamped
//! blend forwarded to the frame's handler) and [`records`] (a query over the
//! frame's record container). [`registry`] says which verified methods are
//! restated, which are proven against their rewrites, and what each proof
//! leaves out.
//!
//! No `unsafe` and no operating-system call in this module. The test crate
//! `lf-layoutframe-diff` compares each method with its verified rewrite on the
//! 32-bit target.

pub mod blend;
pub mod follow_up;
pub mod frame;
pub mod list;
pub mod records;
pub mod registry;

pub use blend::BlendHost;
pub use follow_up::{FollowUpHost, SharedFlag, Target, TargetStep};
pub use frame::{SPAN, UILayoutFrame, field};
pub use list::ListHost;
pub use records::RecordHost;
