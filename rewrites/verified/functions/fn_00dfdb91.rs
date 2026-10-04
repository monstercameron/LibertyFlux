// original: 0x00dfdb91 char_to_lower
// rs03f11: locale-sensitive lowercasing of one character (cdecl/1).
//
// When the locale override flag is clear, folds plain ASCII uppercase to
// lowercase and leaves every other value unchanged; otherwise delegates to
// the locale-aware classifier (cdecl/2, second argument always zero) and
// returns its answer unchanged.
export!(cdecl, rw_rs03f11(ch: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x17AC3C4) != 0 {
            callee_cdecl!(1, u32, ch, 0)
        } else if ch.wrapping_sub(0x41) > 0x19 {
            ch
        } else {
            ch.wrapping_add(0x20)
        }
    }
});
