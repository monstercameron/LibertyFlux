// original: 0x00967B80 append_timing_record
/// Append `(a0, a1)` to the timing record arrays at `this`.
///
/// The record count lives at `this + 0x2D30`; at most 0x3F records fit,
/// so a count whose successor reaches 0x40 ends the function with no
/// effect. Otherwise `a0` is stored at `this + count * 4 + 0x2A30`, the
/// list helper (0x9673E0) runs with the object found at
/// `this + (count + 0xA8C) * 4` and that slot's address as its key, then
/// `a1` is stored at `this + count * 4 + 0x2B30` (the count is re-read
/// after the call, as in the original) and the count is incremented. No
/// return value.
///
/// Original: 0x00967B80 (thiscall, two stack words).

export!(thiscall, rw_00967B80(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x2D30;
        const KEYS: u32 = 0x2A30;
        const VALS: u32 = 0x2B30;
        const SLOT_BIAS: u32 = 0xA8C;
        const CAPACITY: u32 = 0x40;
        let count = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        if count.wrapping_add(1) >= CAPACITY {
            return 0;
        }
        (this.wrapping_add(count.wrapping_mul(4)).wrapping_add(KEYS) as *mut u32)
            .write_unaligned(a0);
        let slot = this.wrapping_add(count.wrapping_add(SLOT_BIAS).wrapping_mul(4));
        let inner = (slot as *const u32).read_unaligned();
        callee_thiscall!(1, u32, inner, slot);
        let count2 = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        (this.wrapping_add(count2.wrapping_mul(4)).wrapping_add(VALS) as *mut u32)
            .write_unaligned(a1);
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(count2.wrapping_add(1));
        0
    }
});
