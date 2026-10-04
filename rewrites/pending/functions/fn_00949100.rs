// original: 0x00949100 slot_clear_triple
/// Clear the 9-byte record (two dwords and a byte) at index `idx` in a
/// 12-byte-stride table. Returns `3 * idx`, the dword index the original
/// leaves in EAX.
export!(thiscall, rw_00949100(obj: *mut u8, idx: u32) -> u32 {
    unsafe {
        let base = obj.add(idx.wrapping_mul(3).wrapping_mul(4) as usize);
        *(base as *mut u32) = 0;
        *(base.add(4) as *mut u32) = 0;
        *base.add(8) = 0;
        idx.wrapping_mul(3)
    }
});
