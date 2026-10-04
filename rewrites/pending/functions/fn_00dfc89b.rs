// original: 0x00dfc89b isdigit
/// `isdigit`: nonzero when `c` is a decimal digit.
///
/// Fast path masks bit `0x04` of the wide ctype table entry; when the
/// locale flag is set the query goes to the locale worker instead.
export!(cdecl, rw_00dfc89b(c: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x17AC3C4) != 0 {
            callee_cdecl!(1, u32, c, 0)
        } else {
            let table = *global::<u32>(0x1058AB8) as *const u16;
            (*table.add(c as usize) as u32) & 4
        }
    }
});
