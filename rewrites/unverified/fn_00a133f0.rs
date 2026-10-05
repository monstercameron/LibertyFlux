// original: 0x00a133f0 mode_table_lookup (proposed)
/// Classify the object, then look up a float from the mode's table.
///
/// Asks the classifier for the object's code (written to a stack slot),
/// then reads the mode byte at `this + 0x22c`: mode 1 selects the first
/// table, 2 the second, 3 the third, indexing by the code; any other mode
/// yields 0.0. The result is returned on the x87 stack. Thiscall, one stack
/// argument.
export!(thiscall, rw_00a133f0(this: u32, obj: u32) -> f32 {
    unsafe {
        const CLASSIFY: u32 = 1;
        const MODE_OFF: u32 = 0x22c;
        const TABLE_A: u32 = 0x00e9ae34;
        const TABLE_B: u32 = 0x00e9ae58;
        const TABLE_C: u32 = 0x00e9ae7c;
        let mut code: u32 = 0;
        callee_stdcall!(CLASSIFY, u32, obj, &mut code as *mut u32 as u32);
        let bits = match ((this + MODE_OFF) as *const u8).read() {
            1 => ((relocated(TABLE_A) + code.wrapping_mul(4)) as *const u32)
                .read_unaligned(),
            2 => ((relocated(TABLE_B) + code.wrapping_mul(4)) as *const u32)
                .read_unaligned(),
            3 => ((relocated(TABLE_C) + code.wrapping_mul(4)) as *const u32)
                .read_unaligned(),
            _ => 0,
        };
        f32::from_bits(bits)
    }
});
