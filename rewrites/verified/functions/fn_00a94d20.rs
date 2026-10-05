// original: 0x00a94d20 fiStreamingDevice::vf2

/// Byte offset of the entry at `base + delta`'s data, and its slot number.
///
/// Resolves the entry; a null resolution gives -1. Otherwise the data
/// callee's answer minus the kind table's slot (kind byte at `+0x04`
/// scaled by 160) is shifted left 11 and stored to the output with a zero
/// second word, and the entry's slot `(entry - table_base) / 24` (signed,
/// magic multiply, truncated to 16 bits) is returned.
///
/// Original: thiscall, two stack arguments (delta, out pointer).
/// Two callees (thiscall, 1/0 args).
lf_checker_rt::export!(thiscall, rw_00a94d20(this: u32, delta: u32, out: u32) -> u32 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const ENT_KIND: u32 = 0x04;
        const KIND_STRIDE: u32 = 160;
        const KIND_TABLE: u32 = 0x012fb44c;
        const TABLE_BASE_G: u32 = 0x012fb3a8;
        const OFFSET_SHIFT: u32 = 11;
        const RESOLVE: u32 = 0;
        const DATA_ADDRESS: u32 = 1;
        const UNRESOLVED: u32 = 0xffff_ffff;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Signed divide by 24 as the original's magic sequence does it.
        fn div24(d: i32) -> i32 {
            let hi = ((d as i64 * 0x2aaaaaabi64) >> 32) as i32;
            let s = hi >> 2;
            s.wrapping_add(((s as u32) >> 31) as i32)
        }
        let t = rd32(this.wrapping_add(BASE_KEY)).wrapping_add(delta);
        let ent: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, t);
        if ent == 0 {
            return UNRESOLVED;
        }
        let data: u32 = lf_checker_rt::callee_thiscall!(DATA_ADDRESS, u32, ent);
        let kind = rd8(ent.wrapping_add(ENT_KIND)) as u32;
        let slot = rd32(lf_checker_rt::relocated(KIND_TABLE)
            .wrapping_add(kind.wrapping_mul(KIND_STRIDE)));
        wr32(out, data.wrapping_sub(slot).wrapping_shl(OFFSET_SHIFT));
        wr32(out.wrapping_add(4), 0);
        let base = rd32(lf_checker_rt::relocated(TABLE_BASE_G));
        div24((ent as i32).wrapping_sub(base as i32)) as u16 as u32
    }
});
