// original: 0x00cc6ba0 best_match_above_floor
/// Scan the kind's candidate table and return the iterated item whose tag
/// (`+0xC`) matches a candidate and whose score (`+0x58`) is the highest
/// above the global floor (null when nothing qualifies).
export!(stdcall, rw_00cc6ba0(obj: u32, kind: u32) -> u32 {
    unsafe {
        let mut table: u32 = 0;
        let count: u32 =
            callee_stdcall!(1, u32, kind, &mut table as *mut u32 as u32);
        let mut best_bits = *global::<u32>(0xFE8D94);
        let mut best_obj: u32 = 0;
        if (count as i32) > 0 {
            let cursor = *(((obj) as *const u8).add(0x78) as *const u32);
            for i in 0..(count as i32) {
                let want = *(((table) as *const u32).add(i as usize));
                let mut item: u32 = callee_thiscall!(2, u32, cursor);
                while item != 0 {
                    if *(((item) as *const u8).add(0xC) as *const u32) == want {
                        let score = f32::from_bits(*(((item) as *const u8).add(0x58)
                            as *const u32));
                        if score > f32::from_bits(best_bits) {
                            best_bits = score.to_bits();
                            best_obj = item;
                        }
                    }
                    item = callee_thiscall!(3, u32, cursor);
                }
            }
        }
        best_obj
    }
});
