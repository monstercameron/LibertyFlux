// original: 0x00afdee0 reset_input_state_defaults
/// Reset the input-state globals to their defaults, run the follow-up
/// initialiser and mark the state block valid.
export!(cdecl, rw_00afdee0() -> u32 {
    unsafe {
        *global::<u32>(0x103ffb4) = 0xffffffff;
        *global::<u32>(0x103ffec) = 0x3f800000;
        *global::<u32>(0x1600184) = 0;
        *global::<u8>(0x160014a) = 0;
        *global::<u8>(0x103fff1) = 1;
        *global::<u8>(0x103fff0) = 1;
        *global::<u8>(0x160017e) = 1;
        *global::<u32>(0x103ffac) = 0x3f800000;
        *global::<u32>(0x103ffb0) = 0x3f800000;
        *global::<u8>(0x103ff75) = 1;
        let answer = callee_cdecl!(1, u32,);
        *global::<u32>(0x160031c) = 0xffffffff;
        answer
    }
});
