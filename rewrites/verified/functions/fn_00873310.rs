// original: 0x00873310 tag_slot_alloc_and_fill
use lf_checker_rt::{callee_thiscall, export};

/// Allocate a tag slot through the pool helper and fill it.
///
/// Calls the pool helper with (this+0x10, this), stores the tag word at
/// the returned slot and copies 16 bytes from the source pointer after it.
/// Returns the slot pointer.
export!(thiscall, rw_00873310(this_: u32, tag: u32, src: u32) -> u32 {
    unsafe {
        let slot = callee_thiscall!(1, u32, this_.wrapping_add(0x10), this_);
        *(slot as *mut u32) = tag;
        *(slot.wrapping_add(4) as *mut u64) = *(src as *const u64);
        *(slot.wrapping_add(12) as *mut u64) = *(src.wrapping_add(8) as *const u64);
        slot
    }
});
