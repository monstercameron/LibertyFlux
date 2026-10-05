// original: 0x00a127b0 mode_index_select (proposed)
/// Map camera state words to a mode index, adjusted by a global selector.
///
/// When the word at `obj + 0x1300` is 1, the result is 1, or 7 when the
/// global selector equals 1. Otherwise the word at `obj + 0x1304` selects:
/// 2 gives 4, 4 gives 2 (or 8 when the selector equals 2), 5 gives 3, and
/// anything else gives 0. Stdcall, one stack argument.
export!(stdcall, rw_00a127b0(obj: u32) -> u32 {
    unsafe {
        const PRIMARY_OFF: u32 = 0x1300;
        const SECONDARY_OFF: u32 = 0x1304;
        const SELECTOR: u32 = 0x011d6fd4;
        if ((obj + PRIMARY_OFF) as *const u32).read_unaligned() == 1 {
            if *global::<u32>(SELECTOR) == 1 {
                7
            } else {
                1
            }
        } else {
            match ((obj + SECONDARY_OFF) as *const u32).read_unaligned() {
                2 => 4,
                4 => {
                    if *global::<u32>(SELECTOR) == 2 {
                        8
                    } else {
                        2
                    }
                }
                5 => 3,
                _ => 0,
            }
        }
    }
});
