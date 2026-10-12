// original: 0x00B3CF30 road_override_remove

/// Removes every road record matching the first cdecl argument, then cleans
/// the objects referenced by the active road-override list.
///
/// The active list pointer and count are globals. Each list row is 12 bytes:
/// the low bit of its first byte enables cleanup, and the next word is the
/// object pointer. Matching records use a separate 48-byte data stride. When
/// a match is not the last record, the following records are shifted down by
/// one row. Float fields are copied as raw words, preserving every bit pattern.
/// A stack guard object is opened with the relocated global token and closed
/// on exit; its frame pointer is passed through the checker runtime.
///
/// Record counts are interpreted as signed values. The active-list count is
/// an unsigned loop bound. The function is cdecl with two stack words; its
/// return register has no documented meaning.
lf_checker_rt::export!(cdecl, rw_00b3cf30(road_key: u32, _unused: u32) -> () {
    const GUARD_TOKEN: u32 = 0x016b_8fa4;
    const ACTIVE_COUNT: u32 = 0x016b_6b38;
    const ACTIVE_LIST: u32 = 0x016b_8f84;
    const RECORD_COUNT: u32 = 0x016b_6afc;
    const RECORD_KEYS: u32 = 0x016b_9054;
    const RECORD_DATA: u32 = 0x016b_9078;
    const ACTIVE_ROW_STRIDE: u32 = 12;
    const RECORD_STRIDE: u32 = 0x30;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn copy_record(source: u32) {
        let word_at_minus_18 = unsafe { read_u32(source.wrapping_sub(0x18)) };
        let word_at_minus_0c = unsafe { read_u32(source.wrapping_sub(0x0c)) };
        let word_at_minus_08 = unsafe { read_u32(source.wrapping_sub(0x08)) };
        let word_at_minus_04 = unsafe { read_u32(source.wrapping_sub(0x04)) };
        let float_at_minus_14 = unsafe { read_u32(source.wrapping_sub(0x14)) };
        let float_at_minus_10 = unsafe { read_u32(source.wrapping_sub(0x10)) };
        let float_at_zero = unsafe { read_u32(source) };
        let word_at_plus_04 = unsafe { read_u32(source.wrapping_add(4)) };
        let word_at_plus_08 = unsafe { read_u32(source.wrapping_add(8)) };
        let word_at_plus_0c = unsafe { read_u32(source.wrapping_add(0x0c)) };
        let old_tag = unsafe { read_u32(source.wrapping_sub(0x28)) };

        unsafe {
            write_u32(source.wrapping_sub(0x48), word_at_minus_18);
            write_u32(source.wrapping_sub(0x44), float_at_minus_14);
            write_u32(source.wrapping_sub(0x40), float_at_minus_10);
            write_u32(source.wrapping_sub(0x3c), word_at_minus_0c);
            write_u32(source.wrapping_sub(0x38), word_at_minus_08);
            write_u32(source.wrapping_sub(0x34), word_at_minus_04);
            write_u32(source.wrapping_sub(0x30), float_at_zero);
            write_u32(source.wrapping_sub(0x2c), word_at_plus_04);
            write_u32(
                source.wrapping_sub(0x28),
                old_tag ^ ((word_at_plus_08 ^ old_tag) & 1),
            );
            write_u32(source.wrapping_sub(0x24), word_at_plus_0c);
        }
    }

    unsafe {
        let mut guard_storage = core::mem::MaybeUninit::<[u32; 20]>::uninit();
        let guard_pointer = guard_storage.as_mut_ptr().cast::<u32>().add(11);
        let guard = guard_pointer as usize as u32;
        let _ = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            guard,
            lf_checker_rt::relocated(GUARD_TOKEN)
        );

        let active_count = read_u32(lf_checker_rt::relocated(ACTIVE_COUNT));
        let active_list = read_u32(lf_checker_rt::relocated(ACTIVE_LIST));
        let mut active_index = 0u32;
        while active_index < active_count {
            let row = active_list.wrapping_add(active_index.wrapping_mul(ACTIVE_ROW_STRIDE));
            if (unsafe { (row as *const u8).read() } & 1) != 0 {
                let object = read_u32(row.wrapping_add(4));
                if object != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(2, u32, object, 1);
                }
            }
            active_index = active_index.wrapping_add(1);
        }

        let record_count_address = lf_checker_rt::relocated(RECORD_COUNT);
        let mut record_count = read_u32(record_count_address) as i32;
        if record_count > 0 {
            let mut last_index = record_count.wrapping_sub(1);
            guard_pointer.write(last_index as u32);
            let record_keys = lf_checker_rt::relocated(RECORD_KEYS);
            let record_data = lf_checker_rt::relocated(RECORD_DATA);
            let mut record_index = 0i32;

            while record_index < record_count {
                let key_address = record_keys.wrapping_add(
                    (record_index as u32).wrapping_mul(RECORD_STRIDE),
                );
                if read_u32(key_address) == road_key {
                    if record_index < last_index {
                        let shifts = (last_index - record_index) as u32;
                        let mut source = record_data.wrapping_add(
                            (record_index as u32).wrapping_mul(RECORD_STRIDE),
                        );
                        let mut remaining = shifts;
                        while remaining != 0 {
                            copy_record(source);
                            source = source.wrapping_add(RECORD_STRIDE);
                            remaining -= 1;
                        }
                    }
                    record_count -= 1;
                    last_index -= 1;
                    guard_pointer.write(last_index as u32);
                } else {
                    record_index += 1;
                }
            }
            write_u32(record_count_address, record_count as u32);
        }

        let cleanup_count = read_u32(lf_checker_rt::relocated(ACTIVE_COUNT));
        let cleanup_list = read_u32(lf_checker_rt::relocated(ACTIVE_LIST));
        let mut cleanup_index = 0u32;
        while cleanup_index < cleanup_count {
            let row = cleanup_list.wrapping_add(cleanup_index.wrapping_mul(ACTIVE_ROW_STRIDE));
            if (unsafe { (row as *const u8).read() } & 1) != 0 {
                let object = read_u32(row.wrapping_add(4));
                if object != 0 {
                    let _ = lf_checker_rt::callee_cdecl!(3, u32, object);
                }
            }
            cleanup_index = cleanup_index.wrapping_add(1);
        }

        let _ = lf_checker_rt::callee_thiscall!(4, u32, guard);
    }
});
