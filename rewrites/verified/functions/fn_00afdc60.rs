// original: 0x00afdc60 classify_handle_by_table
/// Classify a handle against sixteen table slots: 4 for the middle group,
/// 1 for the twelfth slot or no match, 2 everywhere else.
export!(cdecl, rw_00afdc60(arg: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x12f9fb4) == arg { return 2; }
        if *global::<u32>(0x12fa62c) == arg { return 2; }
        if *global::<u32>(0x12f9ff0) == arg { return 2; }
        if *global::<u32>(0x12f9f24) == arg { return 2; }
        if *global::<u32>(0x12fa044) == arg { return 2; }
        if *global::<u32>(0x12fa284) == arg { return 2; }
        if *global::<u32>(0x12fa068) == arg { return 2; }
        if *global::<u32>(0x12f9dbc) == arg { return 2; }
        if *global::<u32>(0x12fa314) == arg { return 4; }
        if *global::<u32>(0x12fa494) == arg { return 4; }
        if *global::<u32>(0x12fa3a4) == arg { return 4; }
        if *global::<u32>(0x12f9e34) == arg { return 1; }
        if *global::<u32>(0x12fa428) == arg { return 2; }
        if *global::<u32>(0x12fa56c) == arg { return 2; }
        if *global::<u32>(0x12fa374) == arg { return 2; }
        if *global::<u32>(0x12f9f30) == arg { return 2; }
        1
    }
});
