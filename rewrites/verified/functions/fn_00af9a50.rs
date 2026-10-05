// original: 0x00AF9A50 veh_pool_count_model (proposed)

/// Count live pool rows whose model word equals the target.
///
/// The pool metadata reached through the global pool pointer holds the row
/// base at `+0x0`, flag bytes at `+0x4`, the row count at `+0x8` and the row
/// stride at `+0xC`. Every row from the last down to the first is visited; a
/// row is skipped when its flag byte has bit 7 set or its computed address
/// is null, and otherwise counts when the signed word at row + 0x2E equals
/// `model`. Returns the count.
///
/// Original: 0x00AF9A50 (cdecl, one stack argument, count in EAX).
lf_checker_rt::export!(cdecl, rw_00AF9A50(model: u32) -> u32 {
    unsafe {
        const POOL_META: u32 = 0x12E22A4;
        const MODEL_WORD: u32 = 0x2E;
        const GONE_BIT: u8 = 0x80;
        let meta = (lf_checker_rt::global::<u32>(POOL_META) as *const u32).read_unaligned();
        let rows = (meta as *const u32).read_unaligned();
        let flags = ((meta + 4) as *const u32).read_unaligned();
        let count = ((meta + 8) as *const u32).read_unaligned();
        let stride = ((meta + 0xC) as *const u32).read_unaligned();
        let mut found = 0u32;
        let mut i = count;
        while i > 0 {
            i -= 1;
            if ((flags.wrapping_add(i)) as *const u8).read() & GONE_BIT != 0 {
                continue;
            }
            let row = rows.wrapping_add(stride.wrapping_mul(i));
            if row == 0 {
                continue;
            }
            let w = ((row + MODEL_WORD) as *const i16).read_unaligned() as i32 as u32;
            if w == model {
                found += 1;
            }
        }
        found
    }
});
