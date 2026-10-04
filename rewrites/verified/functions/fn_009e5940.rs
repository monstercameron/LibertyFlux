// original: 0x009e5940 CPlayerPed::vf69

/// Reset two slots, optionally pre-run, apply, then notify.
///
/// Zeroes `this + 0xd48`/`+ 0xd4c`. When the low nibble of
/// `this + 0x1e2` is 6, 8, 9 or 0xa and bit 0x10 of the argument is
/// clear, calls pre (`thiscall` on this with the argument). Always
/// writes -1 to `this + 0xd50`/`+ 0xd54`, calls apply (`thiscall` on
/// this with the argument), then notify (`thiscall` on `[this + 0x7b0]`
/// with `(8, 0)`), whose answer is returned. `thiscall`, one stack word.
lf_checker_rt::export!(thiscall, rw_009e5940(this: u32, a1: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x1e2;
        const OUT1: u32 = 0xd48;
        const OUT2: u32 = 0xd4c;
        const OUT3: u32 = 0xd50;
        const OUT4: u32 = 0xd54;
        const NOTIFY_OBJ: u32 = 0x7b0;
        const PRE: u32 = 1;
        const APPLY: u32 = 2;
        const NOTIFY: u32 = 3;
        let nib = ((this + MODE) as *const u16).read_unaligned() & 0xf;
        ((this + OUT1) as *mut u32).write_unaligned(0);
        ((this + OUT2) as *mut u32).write_unaligned(0);
        if (nib == 8 || nib == 9 || nib == 0xa || nib == 6) && (a1 & 0x10) == 0 {
            lf_checker_rt::callee_thiscall!(PRE, u32, this, a1);
        }
        ((this + OUT3) as *mut u32).write_unaligned(0xffffffff);
        ((this + OUT4) as *mut u32).write_unaligned(0xffffffff);
        lf_checker_rt::callee_thiscall!(APPLY, u32, this, a1);
        let obj = ((this + NOTIFY_OBJ) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, obj, 8u32, 0u32)
    }
});
