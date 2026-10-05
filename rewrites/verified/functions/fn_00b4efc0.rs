// original: 0x00b4efc0 find_task_by_ids (proposed)

/// Find the first task in a chain matching three identity words.
///
/// The chain head at `this + 0x78` yields its first task (callee 1,
/// thiscall on the head); a null first task means no match. Each task is
/// tested against `a` (word at `+0x14`), `b` (`+0x18`) and `c` (`+0x08`),
/// returning the first task matching all three. Later tasks come from
/// callee 2 (thiscall on the head); a null next task ends the search with
/// no match (null).
///
/// Original: 0x00b4efc0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00b4efc0(this: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x78;
        const FIRST: u32 = 1;
        const NEXT: u32 = 2;
        const KEY_A: u32 = 0x14;
        const KEY_B: u32 = 0x18;
        const KEY_C: u32 = 0x08;
        let head = ((this + HEAD) as *const u32).read_unaligned();
        let mut cur = lf_checker_rt::callee_thiscall!(FIRST, u32, head);
        if cur == 0 {
            return 0;
        }
        loop {
            if ((cur + KEY_A) as *const u32).read_unaligned() == a
                && ((cur + KEY_B) as *const u32).read_unaligned() == b
                && ((cur + KEY_C) as *const u32).read_unaligned() == c
            {
                return cur;
            }
            cur = lf_checker_rt::callee_thiscall!(NEXT, u32, head);
            if cur == 0 {
                return 0;
            }
        }
    }
});
