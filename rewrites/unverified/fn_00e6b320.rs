// original: 0xe6b320 vec3_const_copy_01 (proposed)

/// Copy three consecutive 4-byte constants between globals.
///
/// Three vector-unit loads and stores: each source word is moved
/// to its matching destination word. A plain bit copy, no
/// arithmetic, no arguments (cdecl, empty stack list), no
/// meaningful return value.
lf_checker_rt::export!(cdecl, rw_00e6b320() -> u32 {
    const SRC0: u32 = 0x01b4b2a0;
    const DST0: u32 = 0x016ec790;
    const SRC1: u32 = 0x01b4b2a4;
    const DST1: u32 = 0x016ec794;
    const SRC2: u32 = 0x01b4b2a8;
    const DST2: u32 = 0x016ec798;
    unsafe {
        lf_checker_rt::global::<u32>(DST0).write_unaligned(
            lf_checker_rt::global::<u32>(SRC0).read_unaligned());
        lf_checker_rt::global::<u32>(DST1).write_unaligned(
            lf_checker_rt::global::<u32>(SRC1).read_unaligned());
        lf_checker_rt::global::<u32>(DST2).write_unaligned(
            lf_checker_rt::global::<u32>(SRC2).read_unaligned());
    }
    0
});
