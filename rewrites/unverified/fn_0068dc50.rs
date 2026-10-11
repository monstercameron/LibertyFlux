// original: 0x0068DC50 blend_shape_weighted_record_dispatch_0068DC50 (proposed)

/// Scale nonzero weights and dispatch them through each record's method table.
///
/// The object in ECX points to a component with an unsigned count at `+0x10`
/// and a record-pointer array at `+0x0c`. The first stack argument points to
/// the parallel weight array. The low f32 in XMM2 is the multiplier. Records
/// with flag `0x10` set or a zero weight are skipped; other weights are
/// multiplied by the XMM2 value in that operand order and passed on the stack
/// to the record's vtable slot `+0x1c`. EAX is not a meaningful return value.
lf_checker_rt::export!(thiscall, rw_0068DC50(wrapper: u32, weight_array: u32, _reserved: u32) -> () {
    unsafe {
        const COMPONENT_OFFSET: u32 = 0;
        const RECORD_ARRAY_OFFSET: u32 = 0x0c;
        const RECORD_COUNT_OFFSET: u32 = 0x10;
        const RECORD_POINTER_BYTES: u32 = 4;
        const RECORD_FLAGS_OFFSET: u32 = 4;
        const EXCLUDED_FLAG: u8 = 0x10;
        const VTABLE_SLOT: u32 = 0x1c;

        let component = *((wrapper.wrapping_add(COMPONENT_OFFSET)) as *const u32);
        let count = *((component.wrapping_add(RECORD_COUNT_OFFSET)) as *const u16) as u32;
        let record_array = *((component.wrapping_add(RECORD_ARRAY_OFFSET)) as *const u32);
        let multiplier = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
        let mut record_index = 0u32;
        while record_index < count {
            let record_offset = record_index.wrapping_mul(RECORD_POINTER_BYTES);
            let record = *((record_array.wrapping_add(record_offset)) as *const u32);
            let weight_bits = *((weight_array.wrapping_add(record_offset)) as *const u32);
            let flags = *((record.wrapping_add(RECORD_FLAGS_OFFSET)) as *const u8);
            if flags & EXCLUDED_FLAG == 0 && weight_bits & 0x7fff_ffff != 0 {
                let weight = f32::from_bits(weight_bits);
                let scaled_weight = core::hint::black_box(weight)
                    * core::hint::black_box(multiplier);
                let _ = *((*((record) as *const u32)).wrapping_add(VTABLE_SLOT) as *const u32);
                let _ = lf_checker_rt::callee_thiscall!(1, u32, record, scaled_weight.to_bits());
            }
            record_index += 1;
        }
    }
});
