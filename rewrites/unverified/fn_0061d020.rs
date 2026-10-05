// original: 0x0061D020 net_init_channel

/// Initialize a channel record from seven arguments.
///
/// Stores `a0`/`a1` in the header, runs the sub-initializer over
/// `this+8` with `a2`, copies the address block at `a3` into
/// `this+0x78` (dword, word, dword, word), stores `a6` at `this+0x8C`
/// (running the registrar over `this+0x90` with `a5`/`a6` when `a6`
/// is nonzero), and stores `a4` at `this+0x88`. Returns `a4`.
/// Original: 0x0061D020 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_0061D020(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const SUB_INIT: u32 = 1;
        const REGISTRAR: u32 = 2;
        ((this) as *mut u32).write_unaligned(a0);
        lf_checker_rt::callee_thiscall!(SUB_INIT, u32, this + 8, a2);
        ((this + 4) as *mut u32).write_unaligned(a1);
        ((this + 0x78) as *mut u32).write_unaligned((a3 as *const u32).read_unaligned());
        ((this + 0x7C) as *mut u16)
            .write_unaligned((((a3 + 4) as *const u16).read_unaligned()));
        ((this + 0x80) as *mut u32)
            .write_unaligned((((a3 + 8) as *const u32).read_unaligned()));
        ((this + 0x84) as *mut u16)
            .write_unaligned((((a3 + 0xC) as *const u16).read_unaligned()));
        ((this + 0x8C) as *mut u32).write_unaligned(a6);
        if a6 != 0 {
            lf_checker_rt::callee_cdecl!(REGISTRAR, u32, this + 0x90, a5, a6);
        }
        ((this + 0x88) as *mut u32).write_unaligned(a4);
        a4
    }
});
