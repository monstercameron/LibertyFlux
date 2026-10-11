// original: 0x00626E10 network_record_passes_gate_checks

/// Return the low-byte success result after a sequence of short-circuit checks on a network
/// record. The owner is in ECX and the record is in EDX. Three fastcall helpers run at the
/// original call sites: the first gates entry, the second receives the record at +0x40, and
/// the final one receives it at +0x88. Between helpers, the bitwise AND of words at +0x38 and
/// +0x3C must not equal all ones, and both OR pairs at +0x48/+0x4C and +0x80/+0x84 must be
/// nonzero. The function writes no memory; only AL is its boolean result.
lf_checker_rt::export!(fastcall, rw_00626e10(owner: u32, record: u32) -> u32 {
    unsafe {
        const RECORD_FLAGS_A: u32 = 0x38;
        const RECORD_FLAGS_B: u32 = 0x3C;
        const REQUIRED_LEFT_VALUE: u32 = 0x48;
        const REQUIRED_RIGHT_VALUE: u32 = 0x4C;
        const REQUIRED_LEFT_STATUS: u32 = 0x80;
        const REQUIRED_RIGHT_STATUS: u32 = 0x84;
        const FIRST_HELPER: u32 = 1;
        const SECOND_HELPER: u32 = 2;
        const FINAL_HELPER: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let initial_check: u32 =
            lf_checker_rt::callee_fastcall!(FIRST_HELPER, u32, owner, record);
        if initial_check & 0xFF == 0 {
            return 0;
        }

        let flags = read_u32(record.wrapping_add(RECORD_FLAGS_A))
            & read_u32(record.wrapping_add(RECORD_FLAGS_B));
        if flags == u32::MAX {
            return 0;
        }

        let middle_check: u32 = lf_checker_rt::callee_fastcall!(
            SECOND_HELPER,
            u32,
            owner,
            record.wrapping_add(0x40)
        );
        if middle_check & 0xFF == 0 {
            return 0;
        }

        let value_pair = read_u32(record.wrapping_add(REQUIRED_LEFT_VALUE))
            | read_u32(record.wrapping_add(REQUIRED_RIGHT_VALUE));
        if value_pair == 0 {
            return 0;
        }

        let status_pair = read_u32(record.wrapping_add(REQUIRED_LEFT_STATUS))
            | read_u32(record.wrapping_add(REQUIRED_RIGHT_STATUS));
        if status_pair == 0 {
            return 0;
        }

        let final_check: u32 = lf_checker_rt::callee_fastcall!(
            FINAL_HELPER,
            u32,
            owner,
            record.wrapping_add(0x88)
        );
        u32::from(final_check & 0xFF != 0)
    }
});
