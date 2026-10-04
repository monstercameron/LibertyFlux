// original: 0x00e5d590 zero_table32_e5d590
/// Zeroes a 32-entry table of dwords in `.data`; always returns zero.
export!(cdecl, rw_e5d590() -> u32 {
    const COUNT: usize = 32;
    unsafe {
        let table = global::<u32>(0x019D23A8);
        for i in 0..COUNT {
            *table.add(i) = 0;
        }
    }
    0
});
