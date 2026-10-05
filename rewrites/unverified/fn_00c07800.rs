// original: 0x00c07800 stream_set_slot_limit20
/// Forward `value` to the slot helper for the `index`-th 80-byte slot.
///
/// Computes the slot address as `this[0] + index * 80` (wrapping) and calls
/// the slot helper (thiscall/1) on it with `value`. Returns the helper's
/// answer. Thiscall: `this` in ECX, two stack words, callee cleans 8.
lf_checker_rt::export!(thiscall, {rw}(this: u32, index: u32, value: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        const SLOT_STRIDE: u32 = 80;
        let base = unsafe { (this as *const u32).read_unaligned() };
        let slot = base.wrapping_add(index.wrapping_mul(SLOT_STRIDE));
        lf_checker_rt::callee_thiscall!(HELPER, u32, slot, value)
    }
});
