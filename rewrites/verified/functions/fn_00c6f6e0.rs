// original: 0x00C6F6E0 anim_record_find

/// Scan the animation-record array for a row whose first dword equals the key. The count is an unsigned 16-bit global, records are spaced 0x30 bytes apart, and the return value is the matching row address; zero means absent.
lf_checker_rt::export!(cdecl, rw_0x00c6f6e0(key: u32) -> u32 {
    const RECORD_COUNT_GLOBAL: u32 = 0x016D_D660;
    const RECORD_TABLE_GLOBAL: u32 = 0x016D_D65C;
    const RECORD_STRIDE: usize = 0x30;

    unsafe {
        let count = lf_checker_rt::global::<u16>(RECORD_COUNT_GLOBAL).read_unaligned() as u32;
        let base = lf_checker_rt::global::<u32>(RECORD_TABLE_GLOBAL).read_unaligned() as *const u8;
        let mut index = 0u32;
        while index < count {
            let record = base.wrapping_add(index as usize * RECORD_STRIDE);
            if core::ptr::read_unaligned(record as *const u32) == key {
                return record as u32;
            }
            index += 1;
        }
        0
    }
});
