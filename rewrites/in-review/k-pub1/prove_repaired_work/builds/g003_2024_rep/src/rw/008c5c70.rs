// original: 0x008C5C70 text_hash_index
/// Hash the supplied label through the intercepted helper, then search the
/// active sixteen-byte records by exact key. Return the zero-based match index
/// or 0xffffffff when the active prefix contains no matching key.
lf_checker_rt::export!(thiscall, rw_008c5c70(this: u32, label: u32) -> u32 {
    const HASH_HELPER: u32 = 1;
    const TEXT_RECORD_STRIDE: u32 = 16;
    unsafe {
        let hash = lf_checker_rt::callee_cdecl!(HASH_HELPER, u32, label, 0);
        let count = read_u32(this + 0x268);
        let records = read_u32(this + 0x264);
        for index in 0..count {
            let record = records.wrapping_add(index.wrapping_mul(TEXT_RECORD_STRIDE));
            if (record as *const u32).read_unaligned() == hash { return index; }
        }
        u32::MAX
    }
});

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}
