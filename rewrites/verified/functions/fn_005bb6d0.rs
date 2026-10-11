// original: 0x005BB6D0 select_byte_using_global_mode
/// Select one byte from a word using the global byte-selection mode. Modes
/// 1, 2 and 3 select shifts of 8, 16 and 24 bits; every other mode selects
/// the low byte. The result is the low byte of the shifted word.
///
/// The original is cdecl with one 32-bit argument and an 8-bit result.
lf_checker_rt::export!(cdecl, rw_005bb6d0(value: u32) -> u8 {
    const MODE_VA: u32 = 0x011d_6fd4;
    let mode = unsafe { lf_checker_rt::global::<u32>(MODE_VA).read() };
    let shift = match mode {
        1 => 8,
        2 => 16,
        3 => 24,
        _ => 0,
    };
    (value >> shift) as u8
});
