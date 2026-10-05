// original: 0x00b4e2b0 CDummyPed::vf35

/// Run the dummy ped's two task passes when the config enables them.
///
/// The config block read from game address `CONFIG` carries an enable bit
/// (bit 0 of the byte at `+0x8E8`); when clear nothing happens. When set,
/// the first pass (callee 1, thiscall on `this` with `a` and `b`) runs,
/// then the second pass (callee 2, same shape). The further arguments `c`
/// and `d` are accepted but never read. Always returns 0.
///
/// Original: 0x00b4e2b0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00b4e2b0(this: u32, a: u32, b: u32, _c: u32, _d: u32) -> u32 {
    unsafe {
        const FIRST_PASS: u32 = 1;
        const SECOND_PASS: u32 = 2;
        const CONFIG: u32 = 0x012fb1b8;
        const ENABLE_OFF: u32 = 0x8e8;
        let cfg = lf_checker_rt::global::<u32>(CONFIG).read_unaligned();
        if ((cfg + ENABLE_OFF) as *const u8).read() & 1 != 0 {
            lf_checker_rt::callee_thiscall!(FIRST_PASS, u32, this, a, b);
            lf_checker_rt::callee_thiscall!(SECOND_PASS, u32, this, a, b);
        }
        0
    }
});
