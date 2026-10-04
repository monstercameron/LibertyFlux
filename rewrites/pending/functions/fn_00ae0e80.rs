// original: 0x00ae0e80 ui_watermark_advance
/// Advance the progress watermark to the global mark when the object allows it.
///
/// Resolves the current object through the first sink, then returns early
/// unless the object's flag byte at `+0x328C` is set and the global mark
/// exceeds the stored watermark by more than `0xC8`. On the full path the
/// object's record at `+0x2B48` is handed to the second sink and the watermark
/// is set to the mark. Returns the second sink's negated answer on the full
/// path, otherwise the (dead) global word the original happens to leave in
/// `eax`.
export!(cdecl, rw_00ae0e80() -> u32 {
    unsafe {
        let mark = *global::<u32>(0x1173594);
        let obj = callee_cdecl!(1, u32, 0);
        let dead = *global::<u32>(0x18B7A70);
        if ((obj + 0x328C) as *const u8).read() == 0 {
            return dead;
        }
        let w = *global::<u32>(0x1593B8C);
        if mark <= w.wrapping_add(0xC8) {
            return dead;
        }
        let r = callee_cdecl!(2, u32, obj.wrapping_add(0x2B48));
        *global::<u32>(0x1593B8C) = mark;
        r.wrapping_neg()
    }
});
