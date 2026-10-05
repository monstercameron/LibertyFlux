// original: 0x00883aa0 stream_channel_init (proposed)
/// Initialise a streaming channel object.
///
/// Zeroes every field of the fixed header (`+0x00`, `+0x08`..`+0x10`,
/// `+0x18`..`+0x24`, `+0x48`, `+0x5c`, flag bytes `+0x60`/`+0x62`) except the
/// state word at `+0x04`, which is set to all-ones (`EMPTY`), then publishes
/// the channel capacity (`8`) at `+0x5c` and fills the four slot words at
/// `+0x4c`..`+0x58` with `EMPTY`. Returns the object pointer.
///
/// Original: thiscall, no stack arguments, returns `this` in `eax`.
lf_checker_rt::export!(thiscall, rw_00883aa0(this: u32) -> u32 {
    unsafe {
        const EMPTY: u32 = 0xFFFF_FFFF;
        const SLOT_COUNT: u32 = 8;
        let base = this as *mut u8;
        let w = |off: u32, v: u32| (base.add(off as usize) as *mut u32).write_unaligned(v);
        w(0x00, 0);
        w(0x04, EMPTY);
        w(0x08, 0);
        w(0x0c, 0);
        w(0x10, 0);
        w(0x18, 0);
        w(0x1c, 0);
        w(0x20, 0);
        w(0x24, 0);
        w(0x48, 0);
        w(0x5c, 0);
        base.add(0x60).write(0);
        base.add(0x62).write(0);
        // Publish capacity, then mark all four slot words empty.
        w(0x5c, SLOT_COUNT);
        w(0x4c, EMPTY);
        w(0x50, EMPTY);
        w(0x54, EMPTY);
        w(0x58, EMPTY);
        this
    }
});
