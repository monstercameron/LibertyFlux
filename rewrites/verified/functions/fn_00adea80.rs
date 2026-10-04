// original: 0x00adea80 ui_apply_to_pointer_range
/// Call the worker once per pointer in `[begin, end)`.
///
/// Each visit passes (element address, element value, context) and advances
/// by one dword; returns the last answer. The third slot is reserved and
/// unread; the context rides in the fourth. The range is never empty: an
/// empty range would return the undefined entry EAX, which no rewrite can
/// observe.
export!(cdecl, rw_00adea80(begin: *const u32, end: *const u32, _reserved: u32, ctx: u32) -> u32 {
    unsafe {
        let mut answer: u32 = 0;
        let mut p = begin;
        while p != end {
            answer = callee_cdecl!(1, u32, p as u32, *p, ctx);
            p = p.add(1);
        }
        answer
    }
});
