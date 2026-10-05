// original: 0x00c820e0 scenario_child_clear (proposed)

/// Detach the child object at `this+0x18` and clear the slot.
///
/// A null child returns at once. Otherwise the child's virtual slot `+0x54`
/// runs, the use-count byte at `+0x15` of its result is decremented, and when
/// a watcher sits at `this+0x1c` whose word at `+0x28` has exactly bit `0x100`
/// of mask `0x3c0` set, `c2(child, 0)` is notified. The slot is then cleared.
/// EAX is meaningless on the empty path (entry garbage; both callers ignore
/// the result, one only passes it through to a caller that ignores it), so
/// the contract compares no return channel.
///
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00c820e0(this: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x18;
        const WATCHER_OFF: u32 = 0x1c;
        const STATE_OFF: u32 = 0x28;
        const STATE_MASK: u32 = 0x3c0;
        const STATE_WANT: u32 = 0x100;
        const VT_SLOT: u32 = 0x54;
        const COUNT_OFF: u32 = 0x15;
        const C2: u32 = 2;
        let esi = this;
        let child = ((esi + CHILD_OFF) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }
        let vt = (child as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + VT_SLOT) as *const u32).read_unaligned() as usize);
        let r = hook(child);
        let cnt = ((r + COUNT_OFF) as *mut u8).read();
        ((r + COUNT_OFF) as *mut u8).write(cnt.wrapping_sub(1));
        let watcher = ((esi + WATCHER_OFF) as *const u32).read_unaligned();
        if watcher != 0 {
            let st = ((watcher + STATE_OFF) as *const u32).read_unaligned() & STATE_MASK;
            if st == STATE_WANT {
                lf_checker_rt::callee_stdcall!(C2, u32, child, 0u32);
            }
        }
        ((esi + CHILD_OFF) as *mut u32).write_unaligned(0);
        0
    }
});
