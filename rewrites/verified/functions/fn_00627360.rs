// original: 0x00627360 network_record_append_header_and_validate_payload

/// Append the record's ten-byte header when the owner has not already committed it and the
/// signed capacity check permits it. The owner is in ECX and the record is in EDX. The helper
/// receives the source pointer, 16-bit record count, fixed header length and destination pointer.
/// The used-byte field advances by ten and raises the signed high-water mark when needed. The
/// record count is then checked as unsigned against 512; zero succeeds without a payload call.
/// A nonzero in-range count is passed as eight bytes per entry to the payload validator. The
/// function returns a low-byte boolean in AL.
lf_checker_rt::export!(fastcall, rw_00627360(owner: u32, record: u32) -> u32 {
    unsafe {
        const OWNER_BUFFER: u32 = 0x04;
        const OWNER_CAPACITY: u32 = 0x08;
        const OWNER_USED: u32 = 0x0C;
        const OWNER_HIGH_WATER: u32 = 0x10;
        const OWNER_FLAGS: u32 = 0x18;
        const RECORD_COUNT: u32 = 0x200;
        const HEADER_BYTES: u32 = 10;
        const MAX_RECORDS: u32 = 0x200;
        const APPEND_HEADER: u32 = 1;
        const VALIDATE_PAYLOAD: u32 = 2;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let record_count = read_u16(record.wrapping_add(RECORD_COUNT)) as u32;
        let previous_used = read_u32(owner.wrapping_add(OWNER_USED));
        let next_used = previous_used.wrapping_add(HEADER_BYTES);
        let flags = read_u8(owner.wrapping_add(OWNER_FLAGS));
        let capacity = read_u32(owner.wrapping_add(OWNER_CAPACITY));

        if flags & 1 != 0 || (next_used as i32) > (capacity as i32) {
            return 0;
        }

        let source = read_u32(owner);
        let buffer = read_u32(owner.wrapping_add(OWNER_BUFFER));
        let destination = buffer.wrapping_add(previous_used);
        let _ = lf_checker_rt::callee_cdecl!(
            APPEND_HEADER,
            u32,
            source,
            record_count,
            HEADER_BYTES,
            destination
        );
        write_u32(owner.wrapping_add(OWNER_USED), next_used);

        let high_water = read_u32(owner.wrapping_add(OWNER_HIGH_WATER));
        if (next_used as i32) > (high_water as i32) {
            write_u32(owner.wrapping_add(OWNER_HIGH_WATER), next_used);
        }

        if record_count > MAX_RECORDS {
            return 0;
        }
        if record_count == 0 {
            return 1;
        }

        let payload_ok: u32 = lf_checker_rt::callee_thiscall!(
            VALIDATE_PAYLOAD,
            u32,
            owner,
            record,
            record_count.wrapping_shl(3),
            MAX_RECORDS
        );
        u32::from(payload_ok & 0xFF != 0)
    }
});
