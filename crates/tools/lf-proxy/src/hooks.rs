//! Generated hook table (currently empty: no game targets verified yet).
//!
//! How to add a replacement (full walkthrough in `loader.md`):
//!
//! 1. Write the detour as `extern "C"` (cdecl) or `extern "system"`
//!    (stdcall). Catch panics at the boundary and fall back to the
//!    original on failure.
//! 2. For a thiscall method, build the detour stub with
//!    `lf_hook::adapters::make_thiscall_detour_stub` and register the
//!    *stub* address as the detour.
//! 3. Add one `HookDef`: name, subsystem, target (RVA + expected first
//!    bytes, rebased at run time), detour address.
//!
//! ```ignore
//! use lf_registry::{HookDef, Registry, Target};
//!
//! extern "C" fn my_replacement(a: u32) -> u32 {
//!     let r = std::panic::catch_unwind(|| my_logic(a));
//!     r.unwrap_or_else(|_| call_original(a))
//! }
//!
//! pub fn register_all(reg: &mut Registry) {
//!     reg.add(HookDef {
//!         name: "audio.update".to_string(),
//!         subsystem: "audio".to_string(),
//!         target: Target::Inline {
//!             rva: 0x123456,
//!             expected: vec![0x55, 0x8B, 0xEC],
//!             follow_jumps: true,
//!         },
//!         detour: my_replacement as usize,
//!     });
//! }
//! ```

use lf_registry::Registry;

/// Register every known replacement. Called once during deferred init.
pub fn register_all(_reg: &mut Registry) {
    // No verified game targets yet. Entries will be generated from the
    // anchor table once live addresses are confirmed with the game.
}
