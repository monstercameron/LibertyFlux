// original: 0x00a0b720 mission_cleanup_contains_live_handle
/// Search the manager's 256 primary entries, starting at `this + 4` with a
/// 44-byte stride, for the exact handle and tag. The tag argument is compared
/// as a full 32-bit value against the zero-extended byte at entry offset 0,
/// so values above 255 do not match. Each matching row is checked by the
/// entry-validity helper; a live match returns 1, and exhaustion returns 0.
/// Thiscall with two stack words; only AL is a defined return value.
lf_checker_rt::export!(thiscall, rw_00a0b720(
    this: u32,
    wanted_tag: u32,
    wanted_handle: u32
) -> u32 {
    unsafe {
        const ENTRY_BASE: u32 = 4;
        const ENTRY_STRIDE: u32 = 0x2c;
        const ENTRY_COUNT: u32 = 0x100;
        const ENTRY_TAG: u32 = 0;
        const ENTRY_HANDLE: u32 = 4;
        const ENTRY_IS_VALID: u32 = 0;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        for index in 0..ENTRY_COUNT {
            let entry = this
                .wrapping_add(ENTRY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            if read_u32(entry.wrapping_add(ENTRY_HANDLE)) != wanted_handle
                || u32::from(read_u8(entry.wrapping_add(ENTRY_TAG))) != wanted_tag
            {
                continue;
            }

            let is_valid = lf_checker_rt::callee_thiscall!(
                ENTRY_IS_VALID,
                u32,
                this,
                entry,
                1
            );
            if is_valid & 0xff != 0 {
                return 1;
            }
        }

        0
    }
});
