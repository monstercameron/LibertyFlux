// original: 0x00c8d160 fn_00c8d160_copy_startup_motion_table

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Copy `count` eight-word rows from the global motion table into the caller's
/// buffer, returning the unsigned row count. The table uses a 0x20-byte stride;
/// each row is loaded before its eight payload words are written. The checker
/// supplies explicit global inputs and a disjoint output allocation.
const MOTION_TABLE_COUNT: u32 = 0x016FC674;
const MOTION_TABLE_BASE: u32 = 0x0171B7B0;
const MOTION_TABLE_STRIDE: u32 = 0x20;
const MOTION_TABLE_WORDS: u32 = 8;

unsafe fn copy_startup_motion_table(destination: u32, omit_last_word: bool) -> u32 {
    let count = unsafe { read_u32(lf_checker_rt::relocated(MOTION_TABLE_COUNT)) };
    let source = lf_checker_rt::relocated(MOTION_TABLE_BASE);
    let mut row = 0u32;
    while row < count {
        let source_row = source + row * MOTION_TABLE_STRIDE;
        let destination_row = destination + row * MOTION_TABLE_STRIDE;
        let values = [
            unsafe { read_u32(source_row) },
            unsafe { read_u32(source_row + 4) },
            unsafe { read_u32(source_row + 8) },
            unsafe { read_u32(source_row + 12) },
            unsafe { read_u32(source_row + 16) },
            unsafe { read_u32(source_row + 20) },
            unsafe { read_u32(source_row + 24) },
            unsafe { read_u32(source_row + 28) },
        ];
        let mut word = 0u32;
        while word < MOTION_TABLE_WORDS {
            if !omit_last_word || word + 1 != MOTION_TABLE_WORDS {
                unsafe { write_u32(destination_row + word * 4, values[word as usize]); }
            }
            word += 1;
        }
        row += 1;
    }
    count
}

lf_checker_rt::export!(cdecl, rw_00c8d160_copy_startup_motion_table(destination: u32) -> u32 {
    unsafe { copy_startup_motion_table(destination, false) }
});
