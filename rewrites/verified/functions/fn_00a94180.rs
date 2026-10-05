// original: 0x00a94180 stream_resolve_entry (proposed)

/// Resolve a stream key to its 24-byte entry pointer.
///
/// The key is translated to a slot number by a callee; the entry is the
/// slot-th 24-byte record of the array at `[this+4][0]`. Returns null when
/// the slot is 0 and the array base is 0, i.e. plain `base + slot * 24`
/// with wrapping arithmetic.
///
/// Original: thiscall, one stack argument (key). One callee (cdecl, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a94180(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_LINK: u32 = 0x04;
        const ENTRY_STRIDE: u32 = 24;
        const SLOT_OF_KEY: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let slot: u32 = lf_checker_rt::callee_cdecl!(SLOT_OF_KEY, u32, key);
        let base = rd32(rd32(this.wrapping_add(TABLE_LINK)));
        base.wrapping_add(slot.wrapping_mul(ENTRY_STRIDE))
    }
});
