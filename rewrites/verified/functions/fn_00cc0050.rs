// original: 0x00CC0050 CPedMoveBlendOnFoot::vf3
/// Resolve the blend source by trying the primary helper and, when it returns
/// null, the fallback helper. Write zero and then source words at offsets
/// 0x360, 0x364 and 0x368 into the four-word output record.
lf_checker_rt::export!(thiscall, rw_00cc0050(this: *mut u8, output: *mut u8, _unused: u32) -> u32 {
    const SOURCE_HANDLE_OFFSET: usize = 0x3C;
    const SOURCE_COMPONENTS_OFFSET: usize = 0x360;
    const ZERO_WORD: u32 = 0;
    let source_handle = unsafe { this.add(SOURCE_HANDLE_OFFSET).cast::<u32>().read_unaligned() };
    let source = lf_checker_rt::callee_cdecl!(1, u32, source_handle);
    let source = if source == 0 {
        lf_checker_rt::callee_cdecl!(2, u32, source_handle)
    } else {
        source
    } as *const u8;
    let first = unsafe { source.add(SOURCE_COMPONENTS_OFFSET).cast::<u32>().read_unaligned() };
    let second = unsafe { source.add(SOURCE_COMPONENTS_OFFSET + 4).cast::<u32>().read_unaligned() };
    let third = unsafe { source.add(SOURCE_COMPONENTS_OFFSET + 8).cast::<u32>().read_unaligned() };
    unsafe {
        output.cast::<u32>().write_unaligned(ZERO_WORD);
        output.add(4).cast::<u32>().write_unaligned(first);
        output.add(8).cast::<u32>().write_unaligned(second);
        output.add(12).cast::<u32>().write_unaligned(third);
    }
    third
});
