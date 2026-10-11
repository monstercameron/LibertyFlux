// original: 0x006947E0 rage::crCreatureComponentBlendShapes::vf7

/// Clear each helper object's exclusion bit and reset its value to zero.
///
/// The object in ECX points to a count at `+0x10` and a 20-byte record array
/// at `+0x0c`. Each record's leading word selects a helper object of type
/// `0x15`; the helper receives the first stack argument in ECX. The second
/// stack argument is unused. The method returns the unsigned 16-bit count.
lf_checker_rt::export!(thiscall, rw_006947E0(object: u32, helper_context: u32, _reserved: u32) -> u32 {
    unsafe {
        const RECORDS_OFFSET: u32 = 0x0c;
        const COUNT_OFFSET: u32 = 0x10;
        const RECORD_SIZE: u32 = 0x14;
        const RECORD_INDEX_OFFSET: u32 = 0;
        const RECORD_VALUE_OFFSET: u32 = 4;
        const HELPER_FLAGS_OFFSET: u32 = 4;
        const HELPER_VALUE_OFFSET: u32 = 0x10;
        const EXCLUDED_FLAG: u8 = 0x10;
        const HELPER_TYPE: u32 = 0x15;

        let record_count = *((object.wrapping_add(COUNT_OFFSET)) as *const u16) as u32;
        let record_array = *((object.wrapping_add(RECORDS_OFFSET)) as *const u32);
        let mut record_index = 0u32;
        while record_index < record_count {
            let record = record_array.wrapping_add(record_index.wrapping_mul(RECORD_SIZE));
            let helper_index = *((record.wrapping_add(RECORD_INDEX_OFFSET)) as *const u16) as u32;
            let helper_object = lf_checker_rt::callee_thiscall!(1, u32, helper_context, HELPER_TYPE, helper_index);
            if helper_object != 0 {
                let flags = (helper_object.wrapping_add(HELPER_FLAGS_OFFSET)) as *mut u8;
                *flags &= !EXCLUDED_FLAG;
                *((helper_object.wrapping_add(HELPER_VALUE_OFFSET)) as *mut u32) = 0;
            }
            record_index += 1;
        }
        record_count
    }
});
