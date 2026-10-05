// original: 0x00b4e5d0 task_chain_apply_flag (proposed)

/// Stamp a flag and a radius over a task chain.
///
/// The chain head at `this + 0x78` yields its first task (callee 1,
/// thiscall on the head); a null head or first task ends silently. Each
/// task is then visited: when `flag` is non-zero a task with a zero word at
/// `+0x08` is skipped outright, otherwise bit 4 of the word at `+0x04` is
/// cleared when set and bit 14 is set, and the radius (callee 2, thiscall
/// on the task with the float argument) is applied. The next task comes
/// from callee 3 (thiscall on the head); a null next task ends the walk.
/// No meaningful return value.
///
/// Original: 0x00b4e5d0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b4e5d0(this: u32, radius_bits: u32, flag: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x78;
        const FIRST: u32 = 1;
        const APPLY: u32 = 2;
        const NEXT: u32 = 3;
        const STATE: u32 = 0x04;
        const COUNTER: u32 = 0x08;
        const STALE_BIT: u32 = 0x10;
        const STAMP_BIT: u32 = 0x4000;
        let head = ((this + HEAD) as *const u32).read_unaligned();
        if head == 0 {
            return 0;
        }
        let mut cur = lf_checker_rt::callee_thiscall!(FIRST, u32, head);
        if cur == 0 {
            return 0;
        }
        loop {
            if flag & 0xff == 0 || ((cur + COUNTER) as *const u32).read_unaligned() != 0 {
                let st = (cur + STATE) as *mut u32;
                let v = st.read_unaligned();
                if (v >> 4) & 1 != 0 {
                    st.write_unaligned(v & !STALE_BIT);
                }
                let st = (cur + STATE) as *mut u32;
                st.write_unaligned(st.read_unaligned() | STAMP_BIT);
                lf_checker_rt::callee_thiscall!(APPLY, u32, cur, radius_bits);
            }
            cur = lf_checker_rt::callee_thiscall!(NEXT, u32, head);
            if cur == 0 {
                return 0;
            }
        }
    }
});
