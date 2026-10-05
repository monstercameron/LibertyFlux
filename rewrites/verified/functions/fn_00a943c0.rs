// original: 0x00a943c0 fiStreamingDevice::vf20

/// Two global dwords selected by the entry at `base + delta`'s kind byte.
///
/// Resolves the entry through the slot callee, scales its kind byte at
/// `+0x04` by 160, and returns the dwords at that offset in two adjacent
/// global tables as a 64-bit `edx:eax` pair. A null resolution faults on
/// the kind-byte load, identically on both sides.
///
/// Original: thiscall, one stack argument. One callee (thiscall, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a943c0(this: u32, delta: u32) -> u64 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const ENT_KIND: u32 = 0x04;
        const KIND_STRIDE: u32 = 160;
        const TABLE_LO: u32 = 0x012fb3b8;
        const TABLE_HI: u32 = 0x012fb3bc;
        const RESOLVE: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        let t = rd32(this.wrapping_add(BASE_KEY)).wrapping_add(delta);
        let ent: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, t);
        let off = (rd8(ent.wrapping_add(ENT_KIND)) as u32).wrapping_mul(KIND_STRIDE);
        let lo = rd32(lf_checker_rt::relocated(TABLE_LO).wrapping_add(off));
        let hi = rd32(lf_checker_rt::relocated(TABLE_HI).wrapping_add(off));
        ((hi as u64) << 32) | (lo as u64)
    }
});
