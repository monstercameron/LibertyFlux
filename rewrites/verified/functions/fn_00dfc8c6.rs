// original: 0x00dfc8c6 isspace
/// `isspace`: nonzero when `c` is whitespace.
///
/// Same shape as `isdigit` with table bit `0x08`.
export!(cdecl, rw_00dfc8c6(c: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x17AC3C4) != 0 {
            callee_cdecl!(1, u32, c, 0)
        } else {
            let table = *global::<u32>(0x1058AB8) as *const u16;
            (*table.add(c as usize) as u32) & 8
        }
    }
});
