// original: 0x00685E90 frame_buffer_calc_size (proposed)

/// Count the two active flags in each fixed-size frame record and return the
/// resulting byte count in 32-byte units. The first cdecl argument points to
/// a descriptor whose record count is a zero-extended 16-bit word at `+0x14`
/// and whose contiguous records start at the pointer stored at `+0`. Each
/// record is `0xe0` bytes; byte `+0x1b` contributes one unit and byte `+0x1a`
/// contributes another whenever nonzero. The second cdecl argument adds one
/// to both counts when its low byte is nonzero; the middle argument is unused.
/// A null record pointer skips the loop even when the count is nonzero.
///
/// The original uses a cdecl stack frame with three words and returns
/// `(count_1b + count_1a) << 5` in EAX. The 16-bit count is zero-extended
/// before its signed-positive test, so every nonzero word value is positive.
lf_checker_rt::export!(cdecl, rw_00685E90(descriptor: u32, include_one: u32, _unused: u32) -> u32 {
    unsafe {
        const RECORDS_POINTER: u32 = 0x00;
        const RECORD_COUNT: u32 = 0x14;
        const RECORD_STRIDE: u32 = 0xE0;
        const FLAG_A: u32 = 0x1B;
        const FLAG_B: u32 = 0x1A;
        const UNIT_SHIFT: u32 = 5;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let record_count = unsafe { read_u16(descriptor + RECORD_COUNT) } as u32;
        let records = unsafe { read_u32(descriptor + RECORDS_POINTER) };
        let mut count_a = 0u32;
        let mut count_b = 0u32;

        if record_count != 0 && records != 0 {
            for index in 0..record_count {
                let record = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
                count_a += u32::from(unsafe { read_u8(record + FLAG_A) } != 0);
                count_b += u32::from(unsafe { read_u8(record + FLAG_B) } != 0);
            }
        }

        if include_one as u8 != 0 {
            count_a += 1;
            count_b += 1;
        }

        count_a.wrapping_add(count_b).wrapping_shl(UNIT_SHIFT)
    }
});
