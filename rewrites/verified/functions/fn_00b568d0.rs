// original: 0x00B568D0 crFrameFilterBoneAnalogue::dict_init

/// Create the backing allocator block, initialize a fixed dictionary table
/// when allocation succeeds, store the resulting handle at the object's
/// corresponding global slot, and return that handle. The allocation size is
/// `0x1c`; dictionary creation receives the allocator in ECX and three stack
/// arguments in source order: capacity, relocated table address, and entry
/// size. Both callees are intercepted by the checker.
lf_checker_rt::export!(cdecl, rw_bb568d0() -> u32 {
    const ALLOCATION_BYTES: u32 = 0x1c;
    const ENTRY_CAPACITY: u32 = 0x5dc;
    const TABLE_ADDRESS: u32 = 0x00eaf230;
    const ENTRY_BYTES: u32 = 0x10;
    const RESULT_GLOBAL: u32 = 0x01669d60;

    let allocator = lf_checker_rt::callee_cdecl!(1, u32, ALLOCATION_BYTES);
    let result = if allocator == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            allocator,
            ENTRY_CAPACITY,
            lf_checker_rt::relocated(TABLE_ADDRESS),
            ENTRY_BYTES,
        )
    };
    unsafe { lf_checker_rt::global::<u32>(RESULT_GLOBAL).write_unaligned(result); }
    result
});
