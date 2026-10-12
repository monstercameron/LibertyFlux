// original: 0x00E68B30 initialize_vehicle_light_attribute_table
/// Reproduce initialize_vehicle_light_attribute_table at 0x00e68b30. This zero-argument cdecl routine has no semantic return.
///
/// Initialize 255 records at stride 0x30; each call receives the header pointer at offset -0x24, then the routine writes the fixed all-ones and flag fields.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e68b30() -> () {
    unsafe {
        const BASE: u32 = 0x015f5984;
        const COUNT: usize = 255;
        const STRIDE: u32 = 0x30;
        const HEADER_OFFSET: u32 = 0x24;
        const PREVIOUS_WORD_OFFSET: u32 = 4;
        const LOW_FLAG_OFFSET: u32 = 2;
        const NEXT_FLAG_OFFSET: u32 = 4;
        const BYTE_FLAG_OFFSET: u32 = 6;
        const LOW_FLAG_VALUE: u16 = 0x00ff;
        for index in 0..COUNT {
            let entry = BASE.wrapping_add((index as u32).wrapping_mul(STRIDE));
            let header = entry.wrapping_sub(HEADER_OFFSET);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(header));
            lf_checker_rt::global::<u32>(entry.wrapping_sub(PREVIOUS_WORD_OFFSET)).write(u32::MAX);
            lf_checker_rt::global::<u16>(entry).write(u16::MAX);
            lf_checker_rt::global::<u16>(entry.wrapping_add(LOW_FLAG_OFFSET)).write(LOW_FLAG_VALUE);
            lf_checker_rt::global::<u16>(entry.wrapping_add(NEXT_FLAG_OFFSET)).write(u16::MAX);
            lf_checker_rt::global::<u8>(entry.wrapping_add(BYTE_FLAG_OFFSET)).write(u8::MAX);
        }
    }
});
