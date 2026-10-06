// original: 0x00974f00 audio_voice_active_check (proposed)

/// True (1) when the voice slot is actively bound: the flag byte at +0x11C
/// is nonzero, the bound id at +0x114 is nonzero, and it differs from the
/// current global id. All comparisons are equality against zero or between
/// ids. Returns the byte in AL.
/// Original: 0x00974F00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00974f00(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x11C;
        const ID: u32 = 0x114;
        const GLOBAL_ID: u32 = 0x1165E20;
        if ((this.wrapping_add(FLAG)) as *const u8).read() == 0 {
            return 0;
        }
        let id = ((this.wrapping_add(ID)) as *const u32).read_unaligned();
        if id == 0 {
            return 0;
        }
        if id == lf_checker_rt::global::<u32>(GLOBAL_ID).read_unaligned() {
            return 0;
        }
        1
    }
});
