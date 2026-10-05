// original: 0x00a94d00 fiStreamingDevice::vf1

/// Global slot selected by the entry at `base + delta`'s kind byte.
///
/// Resolves the entry through the slot callee, scales its kind byte at
/// `+0x04` by 160, and returns the dword at that offset in a global table.
/// The second stack argument is read by no instruction (the callee slot
/// signature pops it). A null resolution faults on the kind-byte load,
/// identically on both sides.
///
/// Original: thiscall, two stack arguments (delta, unused).
/// One callee (thiscall, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a94d00(this: u32, delta: u32, _unused: u32) -> u32 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const ENT_KIND: u32 = 0x04;
        const KIND_STRIDE: u32 = 160;
        const KIND_TABLE: u32 = 0x012fb450;
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
        rd32(lf_checker_rt::relocated(KIND_TABLE).wrapping_add(off))
    }
});
