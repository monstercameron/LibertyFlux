// original: 0x006276D0 network_record_validate_full_envelope

/// Validate both fixed header words, reject an empty header, then run the envelope's primary,
/// secondary and optional field checks. The owner is in ECX and the record is in EDX. The count
/// request helper writes an unsigned count at record +0x8C; values above 512 fail, zero succeeds
/// without the payload call, and nonzero counts are passed as eight bytes per entry. The mode
/// argument forwarded to the payload helper is the preserved owner register. The result is an
/// AL boolean.
lf_checker_rt::export!(fastcall, rw_006276d0(owner: u32, record: u32) -> u32 {
    unsafe {
        const CHECK_FIRST_HEADER: u32 = 1;
        const CHECK_SECOND_HEADER: u32 = 2;
        const CHECK_PRIMARY: u32 = 3;
        const CHECK_SECONDARY: u32 = 4;
        const CHECK_FIELD: u32 = 5;
        const REQUEST_COUNT: u32 = 6;
        const CHECK_PAYLOAD: u32 = 7;
        const RECORD_COUNT: u32 = 0x8C;
        const FIRST_PAYLOAD: u32 = 0x90;
        const MAX_ENTRIES: u32 = 0x200;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let first_header: u32 = lf_checker_rt::callee_thiscall!(
            CHECK_FIRST_HEADER,
            u32,
            owner,
            read_u32(record),
            0x20
        );
        if first_header & 0xFF == 0 {
            return 0;
        }

        let second_header: u32 = lf_checker_rt::callee_thiscall!(
            CHECK_SECOND_HEADER,
            u32,
            owner,
            read_u32(record.wrapping_add(4)),
            0x20
        );
        if second_header & 0xFF == 0 {
            return 0;
        }

        if (read_u32(record) | read_u32(record.wrapping_add(4))) == 0 {
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
            owner
        );
        u32::from(payload_ok & 0xFF != 0)
    }
});
