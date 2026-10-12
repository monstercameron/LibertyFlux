// original: 0x00c61630 free_driveby_flag_store_1

/// Releases the pointer stored in the first drive-by flag slot, then clears
/// both adjacent four-byte slots. The pointer is passed to the intercepted C
/// deallocator without being dereferenced by this wrapper. The function has
/// no explicit arguments and no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00c61630() -> () {
    const FIRST_SLOT_VA: u32 = 0x016D_CE5C;
    const SECOND_SLOT_VA: u32 = 0x016D_CE60;

    unsafe {
        let pointer = lf_checker_rt::global::<u32>(FIRST_SLOT_VA).read_unaligned();
        let _ = lf_checker_rt::callee_cdecl!(1, u32, pointer);
        lf_checker_rt::global::<u32>(FIRST_SLOT_VA).write_unaligned(0);
        lf_checker_rt::global::<u32>(SECOND_SLOT_VA).write_unaligned(0);
    }
});
