// original: 0x00627640 network_record_parse_extended_payload

/// Validate an envelope with a fixed sequence of header and field helpers, then request a
/// bounded payload and validate its entries. The owner is in ECX and the record is in EDX. Each
/// helper failure returns false immediately. The request helper writes an unsigned count at
/// record +0x8C; counts above 512 fail, zero succeeds without the final payload call, and other
/// counts pass count*8 bytes and the zero mode to the final helper. The result is a low-byte
/// boolean in AL.
lf_checker_rt::export!(fastcall, rw_00627640(owner: u32, record: u32) -> u32 {
    unsafe {
        const RECORD_HEADER: u32 = 1;
        const CHECK_PRIMARY: u32 = 2;
        const CHECK_SECONDARY: u32 = 3;
        const CHECK_FIELD: u32 = 4;
        const REQUEST_COUNT: u32 = 5;
        const CHECK_PAYLOAD: u32 = 6;
        const RECORD_COUNT: u32 = 0x8C;
        const FIRST_PAYLOAD: u32 = 0x90;
        const MAX_ENTRIES: u32 = 0x200;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let header_ok: u32 =
            lf_checker_rt::callee_fastcall!(RECORD_HEADER, u32, owner, record);
        if header_ok & 0xFF == 0 {
            return 0;
        }

        let primary_ok: u32 = lf_checker_rt::callee_fastcall!(
            CHECK_PRIMARY,
            u32,
            owner,
            record.wrapping_add(8)
        );
        if primary_ok & 0xFF == 0 {
            return 0;
        }

        let secondary_ok: u32 = lf_checker_rt::callee_fastcall!(
            CHECK_SECONDARY,
            u32,
            owner,
            record.wrapping_add(0x78)
        );
        if secondary_ok & 0xFF == 0 {
            return 0;
        }

        let field_ok: u32 = lf_checker_rt::callee_thiscall!(
            CHECK_FIELD,
            u32,
            owner,
            record.wrapping_add(0x88),
            owner
        );
        if field_ok & 0xFF == 0 {
            return 0;
        }

        let request_ok: u32 = lf_checker_rt::callee_thiscall!(
            REQUEST_COUNT,
            u32,
            owner,
            record.wrapping_add(RECORD_COUNT),
            10
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

        let payload_ok: u32 = lf_checker_rt::callee_thiscall!(
            CHECK_PAYLOAD,
            u32,
            owner,
            record.wrapping_add(FIRST_PAYLOAD),
            count.wrapping_shl(3),
            0
        );
        u32::from(payload_ok & 0xFF != 0)
    }
});
