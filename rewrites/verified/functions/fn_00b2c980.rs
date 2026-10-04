// original: 0x00b2c980 garage_model_gate
// 0xB2C980 garage_model_gate (cdecl/1 -> al).
//
// Returns 0 only when the mode global selects the second table and the
// signed model id at arg+0x2e equals one of the two wanted ids; 1 otherwise,
// including a null argument.
export!(cdecl, rw_00b2c980(arg: u32) -> u8 {
    unsafe {
        if *global::<u32>(0x11D6FD4) != 2 {
            return 1;
        }
        if arg == 0 {
            return 1;
        }
        let model = *((arg + 0x2E) as *const i16) as i32;
        if model == *global::<i32>(0x12FA008) {
            return 0;
        }
        if model == *global::<i32>(0x12F9EF4) {
            return 0;
        }
        1
    }
});
