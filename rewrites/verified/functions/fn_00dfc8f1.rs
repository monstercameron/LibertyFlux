// original: 0x00dfc8f1 char_type_flag_test
// rs03f01: locale-sensitive character flag test (cdecl/1).
//
// When the locale override flag is clear, looks the character up in the
// global type table and returns flag bit 0x80 of its entry; otherwise
// delegates to the locale-aware classifier (cdecl/2, second argument always
// zero) and returns its answer unchanged.
export!(cdecl, rw_rs03f01(ch: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x17AC3C4) != 0 {
            callee_cdecl!(1, u32, ch, 0)
        } else {
            let table = *global::<u32>(0x1058AB8) as *const u16;
            (*table.add(ch as usize) as u32) & 0x80
        }
    }
});
