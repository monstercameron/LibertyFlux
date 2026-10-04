// original: 0x00e6af50 float_const_copy (proposed)

/// Copy one 4-byte constant from one global to another.
///
/// Loads a single word through the vector unit and stores it to
/// the destination global; a plain bit copy with no arithmetic.
/// Takes no arguments (cdecl, empty stack list) and returns
/// nothing meaningful.
lf_checker_rt::export!(cdecl, rw_00e6af50() -> u32 {
    const SRC_ADDR: u32 = 0x01048ab4;
    const DST_ADDR: u32 = 0x016d8b14;
    unsafe {
        let v = lf_checker_rt::global::<u32>(SRC_ADDR).read_unaligned();
        lf_checker_rt::global::<u32>(DST_ADDR).write_unaligned(v);
    }
    0
});
