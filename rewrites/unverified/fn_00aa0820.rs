// original: 0x00aa0820 stream_slot_detach (proposed)

/// Detach a slot from both header lists.
///
/// `slot` is removed from the live list at `this + 0x1c0c` first and then
/// from the free list at `this + 0x1c00` (matching the original's order:
/// the second call's answer is the result).
///
/// Original: 0x00aa0820 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00aa0820(this: u32, slot: u32) -> u32 {
    unsafe {
        const LIVE_OFF: u32 = 0x1c0c;
        const FREE_OFF: u32 = 0x1c00;
        const REMOVE_LIVE: u32 = 1;
        const REMOVE_FREE: u32 = 2;
        lf_checker_rt::callee_thiscall!(REMOVE_LIVE, u32, this.wrapping_add(LIVE_OFF), slot);
        lf_checker_rt::callee_thiscall!(REMOVE_FREE, u32, this.wrapping_add(FREE_OFF), slot)
    }
});
