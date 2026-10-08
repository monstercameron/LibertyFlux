//! Accepted complete ring-advance body within the null/declared-callback fixture.
//! The direct helper returns scripted zero; the callback uses a declared slot-five stub.
//! Native helper/callback effects, invalid callback pointers and semantic return type are unknown.
//! Physical EAX, declared state and call ordering were compared; arbitrary engine inputs are outside scope.
//! The earlier missing-runtime-export generation remains excluded; stale rollup fields are not proof.
//! Only mutant scaffolding is removed; positive helper/export bytes are unchanged and projection unbuilt.
//! High-level rewrite for a waveshaper state advance.
//!
//! It asks the shared audio effect helper to advance its scalar table, copies
//! the current 20-byte curve record into the next slot of a three-record ring,
//! stores the new ring index, and returns either the final copied word or the
//! callback's result. The callback is optional.

use core::ptr::{read_unaligned, write_unaligned};
use lf_checker_rt::{callee_thiscall, export};

/// Copy one curve record and notify the optional callback.
///
/// Index and byte-offset arithmetic wrap as 32-bit address calculations,
/// matching the target's pointer-width behavior.
pub(crate) unsafe fn advance_impl(this_: *mut u8, advance_by: u32) -> u32 {
    unsafe {
        let _helper_result = callee_thiscall!(1, u32, this_ as u32);

        let state = read_unaligned(this_.add(0x30) as *const u32);
        let next_state = state.wrapping_add(advance_by) % 3;
        let source_slot = state.wrapping_add(6).wrapping_mul(20);
        let destination_slot = next_state.wrapping_add(6).wrapping_mul(20);
        let source = (this_ as u32).wrapping_add(source_slot) as *const u8;
        let destination = (this_ as u32).wrapping_add(destination_slot) as *mut u8;

        let first = read_unaligned(source as *const u64);
        write_unaligned(destination as *mut u64, first);
        let second = read_unaligned(source.add(8) as *const u64);
        write_unaligned(destination.add(8) as *mut u64, second);
        let final_word = read_unaligned(source.add(16) as *const u32);
        write_unaligned(destination.add(16) as *mut u32, final_word);
        write_unaligned((this_ as u32).wrapping_add(0x30) as *mut u32, next_state);

        let callback = read_unaligned(this_.add(8) as *const *mut u8);
        if callback.is_null() {
            final_word
        } else {
            callee_thiscall!(2, u32, callback as u32)
        }
    }
}

export!(thiscall, rw_waveshaper_advance(this_: *mut u8) -> u32 {
    unsafe { advance_impl(this_, 1) }
});

