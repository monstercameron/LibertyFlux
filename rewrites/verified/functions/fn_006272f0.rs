// original: 0x006272F0 network_record_validate_list_entries

/// Validate the owner/record header, request the count stored at record +8, then check that
/// many eight-byte entries starting at record +0x10. The count helper writes the unsigned
/// count through its pointer; the function rejects values above 32 before walking entries.
/// Each entry helper receives the owner and the current entry pointer, and the first false
/// result returns false. A zero count succeeds. The original uses an integer local as its
/// success flag, which is reflected by the boolean return; the contract disables stack-content
/// comparison and keeps stack adjustment, calls, return, and heap writes checked.
lf_checker_rt::export!(fastcall, rw_006272f0(owner: u32, record: u32) -> u32 {
    unsafe {
        const COUNT_FIELD: u32 = 8;
        const FIRST_ENTRY: u32 = 0x10;
        const ENTRY_STRIDE: u32 = 8;
        const MAX_ENTRIES: u32 = 32;
        const REQUEST_SIZE: u32 = 6;
        const CHECK_HEADER: u32 = 1;
        const REQUEST_COUNT: u32 = 2;
        const CHECK_ENTRY: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let header_ok: u32 = lf_checker_rt::callee_fastcall!(CHECK_HEADER, u32, owner, record);
        if header_ok & 0xFF == 0 {
            return 0;
        }

        let count_address = record.wrapping_add(COUNT_FIELD);
        let request_ok: u32 = lf_checker_rt::callee_thiscall!(
            REQUEST_COUNT,
            u32,
            owner,
            count_address,
            REQUEST_SIZE
        );
        if request_ok & 0xFF == 0 {
            return 0;
        }

        let count = read_u32(count_address);
        if count > MAX_ENTRIES {
            return 0;
        }

        let mut index = 0u32;
        while index < count {
            let entry = record
                .wrapping_add(FIRST_ENTRY)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let entry_ok: u32 =
                lf_checker_rt::callee_fastcall!(CHECK_ENTRY, u32, owner, entry);
            if entry_ok & 0xFF == 0 {
                return 0;
            }
            index = index.wrapping_add(1);
        }
        1
    }
});
