// original: 0x009ddf00 oneshot_table_init
// fn_009ddf00: one-shot table init (cdecl/0).
//
// The first call with a nonzero entry count registers the table through the
// setup step, then latches the done flag so later calls do nothing.
export!(cdecl, rw_009ddf00() -> u32 {
    unsafe {
        if *global::<u8>(0x103AE84) != 0 {
            return 0;
        }
        let count = *global::<u16>(0x103AE8C);
        if count != 0 {
            callee_cdecl!(
                1,
                u32,
                *global::<u32>(0x103AE88),
                count as u32,
                8,
                relocated(0x9DDEE0)
            );
        }
        *global::<u8>(0x103AE84) = 1;
        0
    }
});
