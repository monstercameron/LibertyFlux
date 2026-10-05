// original: 0x008B3CE0 rage::audDelayEffectPc::vf1 (merged symbol)

/// Release the delay line buffer if one is attached.
///
/// Loads `this+0x50`; when nonzero it is passed to the free routine
/// (`0x401250`, cdecl) and the slot is nulled. No return value. Original is
/// thiscall with no stack words (plain `ret`).
lf_checker_rt::export!(thiscall, rw_008B3CE0(this: u32) -> u32 {
    const FREE: u32 = 1;
    const BUFFER: u32 = 0x50;
    unsafe {
        let p = ((this + BUFFER) as *const u32).read_unaligned();
        if p != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, p);
            ((this + BUFFER) as *mut u32).write_unaligned(0);
        }
    }
    0
});
