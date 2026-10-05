// original: 0x009a8ff0 audio_table_shutdown
/// Shut down every live entry of the global audio handle table.
///
/// Walks the 16 entries from 0x1284694 to 0x12847d4 in steps of 20
/// bytes. An entry whose handle word (at the cursor) and key word
/// (4 bytes before it) are both non-null is polled (stubbed,
/// thiscall/2 with `(key, 1)`); answers 1 and 2 mean stopped, and a
/// stopped entry is released (stubbed, thiscall/1 with 0) and its
/// handle word cleared. Cdecl, no arguments, no result.
export!(cdecl, rw_009A8FF0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x1284694;
        const END: u32 = 0x12847d4;
        const STRIDE: u32 = 0x14;
        let mut p = relocated(TABLE);
        let end = relocated(END);
        while p < end {
            let handle = (p as *const u32).read_unaligned();
            if handle != 0 {
                let key = ((p - 4) as *const u32).read_unaligned();
                if key != 0 {
                    let ans: u32 = callee_thiscall!(1, u32, handle, key, 1);
                    if ans == 2 || ans == 1 {
                        let h2 = (p as *const u32).read_unaligned();
                        let _: u32 = callee_thiscall!(2, u32, h2, 0);
                        (p as *mut u32).write_unaligned(0);
                    }
                }
            }
            p += STRIDE;
        }
        0
    }
});
