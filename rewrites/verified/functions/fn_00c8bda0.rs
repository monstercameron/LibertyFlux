// original: 0x00c8bda0 audio_slot_clear (proposed)
///
/// Zeroes five double-words at offsets 0x10..0x20 of the object at `this`
/// and returns `this`. Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c8bda0(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this.wrapping_add(0x10), 0);
        wr32(this.wrapping_add(0x14), 0);
        wr32(this.wrapping_add(0x18), 0);
        wr32(this.wrapping_add(0x1c), 0);
        wr32(this.wrapping_add(0x20), 0);
        this
    }
});
