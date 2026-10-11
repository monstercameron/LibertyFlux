// original: 0x009B6300 script_copy_paired_position_records

/// Copy two four-word position records from the selected script entry into
/// caller-provided output slots. The manager lookup uses selector 27 and
/// secondary selector 0. A null entry leaves both outputs untouched and
/// returns false in AL. Otherwise, words at entry offsets `+0x40..+0x4c` go to
/// the first output and words at `+0x20..+0x2c` go to the second. The vector
/// fields are copied as raw bits, so float payloads are preserved unchanged.
lf_checker_rt::export!(stdcall, rw_009b6300(first_output: u32, second_output: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e49c;
        const ENTRY_SELECTOR: u32 = 27;
        const SECONDARY_SELECTOR: u32 = 0;
        const LOOKUP: u32 = 1;
        const FIRST_FIELDS: [u32; 4] = [0x40, 0x44, 0x48, 0x4c];
        const SECOND_FIELDS: [u32; 4] = [0x20, 0x24, 0x28, 0x2c];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn copy_words(destination: u32, record: u32, fields: [u32; 4]) {
            unsafe {
                for (index, source_offset) in fields.into_iter().enumerate() {
                    let bits = read_u32(record.wrapping_add(source_offset));
                    write_u32(destination.wrapping_add((index as u32) * 4), bits);
                }
            }
        }

        let manager = read_u32(lf_checker_rt::relocated(MANAGER_GLOBAL));
        let record = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            manager,
            ENTRY_SELECTOR,
            SECONDARY_SELECTOR
        );
        if record == 0 {
            0
        } else {
            copy_words(first_output, record, FIRST_FIELDS);
            copy_words(second_output, record, SECOND_FIELDS);
            1
        }
    }
});
