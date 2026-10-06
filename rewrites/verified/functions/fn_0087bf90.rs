// original: 0x0087bf90 rage::crmtNodeProxy::vf4
/// Dispatch a proxy poll: empty child takes the fallback slot.
///
/// With no child at `this+0x24`, tails to slot 0x48 of the argument's table
/// and returns its answer. Otherwise polls slot 0x1c of the child's table;
/// when the poll answers non-null, polls again and feeds that answer to slot
/// 0x0c of the argument's table, then always feeds the child itself to slot
/// 0x10 of the argument's table and returns that answer. All comparisons
/// are null checks.
///
/// Original: thiscall/1, four indirect calls, no floating point.
export!(thiscall, rw_0087bf90(this: u32, obj: u32) -> u32 {
    /// Child link, null-checked for the fallback path.
    const CHILD_OFF: u32 = 0x24;
    /// Poll slot in the child's table.
    const POLL_SLOT: u32 = 0x1C;
    /// Feed slot in the argument's table (takes the re-poll answer).
    const FEED_SLOT: u32 = 0x0C;
    /// Child slot in the argument's table (takes the child).
    const CHILD_SLOT: u32 = 0x10;
    /// Fallback slot in the argument's table (empty child).
    const EMPTY_SLOT: u32 = 0x48;
    unsafe {
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if child == 0 {
            let vt = (obj as *const u32).read_unaligned();
            let tgt = ((vt + EMPTY_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            return f(obj);
        }
        let cvt = (child as *const u32).read_unaligned();
        let poll = ((cvt + POLL_SLOT) as *const u32).read_unaligned();
        let fp: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(poll as usize);
        let r1 = fp(child);
        if r1 != 0 {
            let cvt2 = (child as *const u32).read_unaligned();
            let poll2 = ((cvt2 + POLL_SLOT) as *const u32).read_unaligned();
            let fp2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(poll2 as usize);
            let r2 = fp2(child);
            let ovt = (obj as *const u32).read_unaligned();
            let tgt = ((ovt + FEED_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, r2);
        }
        let ovt = (obj as *const u32).read_unaligned();
        let tgt = ((ovt + CHILD_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(obj, child)
    }
});
