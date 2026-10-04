// original: 0x00cc6fa0 table_contains_iter_tag
/// Return whether any iterated item's tag (`+0xC`) appears in the kind's
/// candidate table: low byte 1 when found (upper bits echo the tag), 0
/// otherwise.
export!(stdcall, rw_00cc6fa0(obj: u32, kind: u32) -> u32 {
    unsafe {
        let mut table: u32 = 0;
        let count: u32 =
            callee_stdcall!(1, u32, kind, &mut table as *mut u32 as u32);
        let cursor = *(((obj) as *const u8).add(0x78) as *const u32);
        let mut item: u32 = callee_thiscall!(2, u32, cursor);
        while item != 0 {
            let tag = *(((item) as *const u8).add(0xC) as *const u32);
            if (count as i32) > 0 {
                for i in 0..(count as i32) {
                    if *(((table) as *const u32).add(i as usize)) == tag {
                        return (tag & 0xFFFF_FF00) | 1;
                    }
                }
            }
            item = callee_thiscall!(3, u32, cursor);
        }
        0
    }
});
