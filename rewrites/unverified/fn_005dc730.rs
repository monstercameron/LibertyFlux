// original: 0x005DC730 ped_task_static_table_setup

/// Register two static task metadata tables and their initial entries.
///
/// The initializer allocates each table descriptor through the same helper,
/// zeroes the three-word descriptor passed to that helper, and stores the
/// returned descriptor pointers in two global slots. It then sends the
/// first descriptor's initial configuration calls in source order. The
/// descriptor pointers passed to the intercepted allocator are stack-local;
/// their contents are compared at each call. This is a cdecl, no-argument
/// initializer with no semantic return value.
lf_checker_rt::export!(cdecl, rw_005dc730() -> () {
    const NAME_FIRST: u32 = 0x00F917E4;
    const NAME_SECOND: u32 = 0x00F917CC;
    const FIRST_TABLE_SLOT: u32 = 0x0103449C;
    const SECOND_TABLE_SLOT: u32 = 0x010344A0;
    const FIRST_ENTRY_FLOAT_BITS: u32 = 0x3F333333;

    let mut first_descriptor = [0u32; 3];
    let first_name = lf_checker_rt::relocated(NAME_FIRST);
    let first_table = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        1u32,
        4u32,
        first_descriptor.as_mut_ptr() as u32,
        2u32,
        first_name,
        0u32,
    );
    unsafe { lf_checker_rt::global::<u32>(FIRST_TABLE_SLOT).write_unaligned(first_table) };

    let _ = lf_checker_rt::callee_cdecl!(2, u32, 3u32, first_table, 6u32);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, 0x0Fu32, first_table, 4u32);
    let _ = lf_checker_rt::callee_cdecl!(
        3,
        u32,
        0u32,
        first_table,
        FIRST_ENTRY_FLOAT_BITS,
    );

    let mut second_descriptor = [0u32; 3];
    let second_name = lf_checker_rt::relocated(NAME_SECOND);
    let second_table = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        1u32,
        4u32,
        second_descriptor.as_mut_ptr() as u32,
        2u32,
        second_name,
        0u32,
    );
    unsafe { lf_checker_rt::global::<u32>(SECOND_TABLE_SLOT).write_unaligned(second_table) };

    let _ = lf_checker_rt::callee_cdecl!(2, u32, 3u32, second_table, 7u32);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, 0x0Fu32, second_table, 0u32);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, 1u32, second_table, 5u32);
});
