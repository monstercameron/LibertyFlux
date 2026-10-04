// original: 0x00566200 race88_vf8_value_width

/// Storage width of a leaderboard row's value kind.
///
/// `index` selects a row of the leaderboard identified by `LEADERBOARD_ID`.
/// The loader callee fills `info` (row table pointer at `+0x14`); the row's
/// value goes through the kind callee, whose answer picks a width: 1 and 5
/// mean 4 bytes, 2 and 3 mean 8 bytes, anything else (including loader or
/// kind failure) means 0.
///
/// Edge cases: loader failure, kind answer -1, and any kind answer outside
/// 1..=5 all return 0.
///
/// Original: thiscall, one stack word; incoming ECX is ignored.
lf_checker_rt::export!(thiscall, rw_00566200(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x123;
        const CALLEE_FILL: u32 = 1;
        const CALLEE_KIND: u32 = 2;
        const INFO_ROWS: usize = 5; // +0x14: row table

        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_FILL, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let rows = info[INFO_ROWS];
        let v = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(CALLEE_KIND, u32, v);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
