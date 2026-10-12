// original: 0x00a0b390 ScriptHandle_CheckOwned
/// Search the 200 secondary mission-cleanup records for `wanted_handle`, the
/// low byte of `wanted_kind`, and the owner identifier at `owner_context+4`.
/// The last exact match is cleared by setting its kind to zero, handle to
/// `u32::MAX`, and owner to zero. Matching handle/kind records with a
/// different owner set a result flag, even if an exact match is found later.
/// The bool return is true only when no different owner was encountered.
/// Thiscall with three stack words; handle equality is bitwise and kind is an
/// unsigned byte comparison.
lf_checker_rt::export!(thiscall, rw_00a0b390(
    this: u32,
    wanted_handle: u32,
    wanted_kind: u32,
    owner_context: u32
) -> u32 {
    unsafe {
        const SECONDARY_BASE: u32 = 0x2c04;
        const ENTRY_STRIDE: u32 = 0x2c;
        const ENTRY_COUNT: u32 = 200;
        const ENTRY_KIND: u32 = 0;
        const ENTRY_HANDLE: u32 = 4;
        const ENTRY_OWNER: u32 = 8;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let owner_id = read_u32(owner_context.wrapping_add(4));
        let wanted_kind = wanted_kind as u8;
        let mut different_owner_seen = false;
        let mut exact_match = None;

        for index in 0..ENTRY_COUNT {
            let entry = this
                .wrapping_add(SECONDARY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            if read_u32(entry.wrapping_add(ENTRY_HANDLE)) != wanted_handle
                || read_u8(entry.wrapping_add(ENTRY_KIND)) != wanted_kind
            {
                continue;
            }

            if read_u32(entry.wrapping_add(ENTRY_OWNER)) == owner_id {
                exact_match = Some(entry);
                continue;
            }
            different_owner_seen = true;
        }

        if let Some(entry) = exact_match {
            write_u32(entry.wrapping_add(ENTRY_HANDLE), u32::MAX);
            write_u8(entry.wrapping_add(ENTRY_KIND), 0);
            write_u32(entry.wrapping_add(ENTRY_OWNER), 0);
        }

        u32::from(!different_owner_seen)
    }
});
