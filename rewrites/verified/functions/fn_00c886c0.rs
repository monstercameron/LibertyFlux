// original: 0x00c886c0 audio_tracker_init (proposed)
///
/// Zeroes nine double-words of the object at `this` (offsets 0x00, 0x04,
/// 0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20) and returns `this`.
/// Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c886c0(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this.wrapping_add(0x08), 0);
        wr32(this.wrapping_add(0x20), 0);
        wr32(this.wrapping_add(0x00), 0);
        wr32(this.wrapping_add(0x0c), 0);
        wr32(this.wrapping_add(0x10), 0);
        wr32(this.wrapping_add(0x14), 0);
        wr32(this.wrapping_add(0x18), 0);
        wr32(this.wrapping_add(0x1c), 0);
        wr32(this.wrapping_add(0x04), 0);
        this
    }
});
