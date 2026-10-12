// original: 0x00685F00 frame_entries_write_pair (proposed)

/// Find two destination frame records using the receiver and key arguments,
/// then copy raw record words from `source`. In full-pair mode both records
/// must exist before either is written; in partial mode each existing record
/// is updated independently. The destination flag byte at `+4` has bit `0x10`
/// cleared. Source offsets `0..16` feed the second record at `+0x10`, while
/// source offsets `+0x10..+0x20` feed the first record at the same offsets.
/// The return is a one-byte success value in AL; upper EAX bytes are inherited
/// from the second lookup. All copied fields are raw 32-bit words, including
/// the two float-valued record fields.
lf_checker_rt::export!(thiscall, rw_00685F00(
    receiver: u32,
    first_key: u32,
    second_key: u32,
    shared_key: u32,
    source: u32,
    allow_partial: u32,
) -> u32 {
    unsafe {
        const RECORD_FLAG: u32 = 0x04;
        const RECORD_PAYLOAD: u32 = 0x10;
        const SOURCE_SECOND_PAYLOAD: u32 = 0x00;
        const SOURCE_FIRST_PAYLOAD: u32 = 0x10;
        const FLAG_COPY_DISABLED: u8 = 0x10;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn clear_copy_disabled_flag(record: u32) {
            let flags = unsafe { read_u8(record.wrapping_add(RECORD_FLAG)) };
            unsafe { write_u8(record.wrapping_add(RECORD_FLAG), flags & !FLAG_COPY_DISABLED) };
        }

        let first_record = unsafe {
            lf_checker_rt::callee_thiscall!(1, u32, receiver, first_key, shared_key)
        };
        let second_record = unsafe {
            lf_checker_rt::callee_thiscall!(2, u32, receiver, second_key, shared_key)
        };
        let partial = allow_partial as u8 != 0;
        if !partial && (first_record == 0 || second_record == 0) {
            return 0;
        }

        if first_record != 0 {
            for word in 0..4u32 {
                let offset = SOURCE_FIRST_PAYLOAD + word * 4;
                let value = unsafe { read_u32(source.wrapping_add(offset)) };
                unsafe { write_u32(first_record.wrapping_add(offset), value) };
            }
            unsafe { clear_copy_disabled_flag(first_record) };
        }
        if second_record != 0 {
            for word in 0..4u32 {
                let source_offset = SOURCE_SECOND_PAYLOAD + word * 4;
                let destination_offset = RECORD_PAYLOAD + word * 4;
                let value = unsafe { read_u32(source.wrapping_add(source_offset)) };
                unsafe { write_u32(second_record.wrapping_add(destination_offset), value) };
            }
            unsafe { clear_copy_disabled_flag(second_record) };
        }

        let any_record = first_record != 0 || second_record != 0;
        let both_records = first_record != 0 && second_record != 0;
        u32::from(if partial { any_record } else { both_records })
    }
});
