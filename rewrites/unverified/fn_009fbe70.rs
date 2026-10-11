// original: 0x009fbe70 play_stat_indexed_init_009fbe70

/// Initialize a temporary integer play-stat record for a signed index.
/// Nonnegative indices select a record from the global pointer table and pass
/// its word at offset 0x3C with the entry's label. Negative indices use the
/// fallback text path, then both paths submit a 0x38-byte object and destroy
/// the temporary base. The input is a signed 32-bit stack argument; the
/// stack-cookie helper's register argument is intentionally not compared.
lf_checker_rt::export!(cdecl, rw_009fbe70(index: i32) -> u32 {
    const DESCRIPTOR_VA: u32 = 0x00e99878;
    const INDEXED_TABLE_VA: u32 = 0x01295cd8;
    const VALUE_FIELD_OFFSET: u32 = 0x3c;
    const TEMP_WORD_OFFSET: usize = 5;
    const AUX_WORD_OFFSET: usize = 19;
    const POSITIVE_LABEL: u32 = 0x00e99612;
    const NEGATIVE_FIRST: u32 = 0x00e996c8;
    const NEGATIVE_SECOND: u32 = 0x00e996d0;
    const EMIT_BYTES: u32 = 0x38;
    unsafe {
        let positive_label = lf_checker_rt::relocated(POSITIVE_LABEL);
        let negative_first = lf_checker_rt::relocated(NEGATIVE_FIRST);
        let negative_second = lf_checker_rt::relocated(NEGATIVE_SECOND);
        let mut local_frame = [0u32; 24];
        let frame_base = local_frame.as_mut_ptr();
        let temporary = frame_base.add(TEMP_WORD_OFFSET);
        let auxiliary = frame_base.add(AUX_WORD_OFFSET);

        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, temporary as u32, 1, 42);
        temporary.write(lf_checker_rt::relocated(DESCRIPTOR_VA));
        let selected_triplet = lf_checker_rt::callee_cdecl!(2, u32, frame_base as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, selected_triplet, auxiliary as u32);

        let (selected_value, selected_label) = if index >= 0 {
            let table = lf_checker_rt::relocated(INDEXED_TABLE_VA) as *const u32;
            let object = table.wrapping_add(index as usize).read_unaligned();
            let value = (object.wrapping_add(VALUE_FIELD_OFFSET) as *const u32).read_unaligned();
            (value, positive_label)
        } else {
            let fallback_value = lf_checker_rt::callee_cdecl!(4, u32, negative_second, 0, negative_first);
            (fallback_value, negative_first)
        };

        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, temporary as u32, selected_value, selected_label);
        let _: u32 = lf_checker_rt::callee_cdecl!(6, u32, temporary as u32, EMIT_BYTES);
        let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, temporary as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, 0);
        0
    }
});
