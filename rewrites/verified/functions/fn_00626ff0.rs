// original: 0x00626FF0 network_record_validate_alternate_entry_list

/// Validate a record header, request its bounded entry list, then check each 16-byte entry.
/// The owner is in ECX and the record is in EDX. A fastcall header helper runs first. The
/// unsigned count at record +0x290 must be at most 32; a thiscall request helper then receives
/// that count pointer and the fixed request size 6. A zero count succeeds without entry calls.
/// Otherwise each entry at record +0x298 + index*0x10 is passed to a fastcall checker, and
/// the first false answer stops the walk. The result is a low-byte boolean in AL.
lf_checker_rt::export!(fastcall, rw_00626ff0(owner: u32, record: u32) -> u32 {
    unsafe {
        const RECORD_COUNT: u32 = 0x290;
        const FIRST_ENTRY: u32 = 0x298;
        const ENTRY_STRIDE: u32 = 0x10;
        const MAX_ENTRIES: u32 = 0x20;
        const REQUEST_SIZE: u32 = 6;
        const FETCH_HEADER: u32 = 1;
        const REQUEST_BYTES: u32 = 2;
        const CHECK_ENTRY: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let header_ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH_HEADER,
            u32,
            owner,
            record
        );
        if header_ok & 0xFF == 0 {
            return 0;
        }

        let request_ok: u32 = lf_checker_rt::callee_thiscall!(
            REQUEST_BYTES,
            u32,
            owner,
            record.wrapping_add(RECORD_COUNT),
            REQUEST_SIZE
        );
        if request_ok & 0xFF == 0 {
            return 0;
        }

        let count = read_u32(record.wrapping_add(RECORD_COUNT));
        if count > MAX_ENTRIES {
            return 0;
        }

        if count == 0 {
            return 1;
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
