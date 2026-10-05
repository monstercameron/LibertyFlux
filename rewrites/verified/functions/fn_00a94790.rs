// original: 0x00a94790 fiStreamingDevice::vf29

/// First word of the gated entry at `base + delta`: -1 written, 0 if gated.
///
/// Resolves the entry through the slot callee. A null resolution or a
/// clear gate bit (bit 13 of the flag word at `+0x0e`) gives 0 with nothing
/// written. Otherwise the entry's first word is stored to the output and
/// the result is -1.
///
/// Original: thiscall, two stack arguments (delta, out pointer).
/// One callee (thiscall, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a94790(this: u32, delta: u32, out: u32) -> u32 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const ENT_FLAGS: u32 = 0x0e;
        const GATE_BIT: u32 = 13;
        const RESOLVE: u32 = 0;
        const FOUND: u32 = 0xffff_ffff;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let t = rd32(this.wrapping_add(BASE_KEY)).wrapping_add(delta);
        let ent: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, t);
        if ent == 0 {
            return 0;
        }
        if (rd16(ent.wrapping_add(ENT_FLAGS)) >> GATE_BIT) & 1 == 0 {
            return 0;
        }
        wr32(out, rd32(ent));
        FOUND
    }
});
