// original: 0x00c82f60 scenario_child_attach (proposed)

/// Attach `new` as the child at `this+0x18`, replacing whatever sat there.
///
/// First `c1(this)` detaches the previous child, then the slot takes `new`. A
/// null child ends the call, returning `c1`'s result. Otherwise the new
/// child's virtual slot `+0x54` runs, the use-count byte at `+0x15` of its
/// result is incremented, and when a watcher sits at `this+0x1c` whose word
/// at `+0x28` has exactly bit `0x100` of mask `0x3c0` set, `c2(child, 1)` is
/// notified and its result returned; without the notify the hook's result is
/// returned. EAX is defined on every path.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c82f60(this: u32, new: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x18;
        const WATCHER_OFF: u32 = 0x1c;
        const STATE_OFF: u32 = 0x28;
        const STATE_MASK: u32 = 0x3c0;
        const STATE_WANT: u32 = 0x100;
        const VT_SLOT: u32 = 0x54;
        const COUNT_OFF: u32 = 0x15;
        const C1: u32 = 1;
        const C3: u32 = 3;
        let esi = this;
        let r0: u32 = lf_checker_rt::callee_thiscall!(C1, u32, esi);
        ((esi + CHILD_OFF) as *mut u32).write_unaligned(new);
        if new == 0 {
            return r0;
        }
        let vt = (new as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + VT_SLOT) as *const u32).read_unaligned() as usize);
        let r1 = hook(new);
        let cnt = ((r1 + COUNT_OFF) as *mut u8).read();
        ((r1 + COUNT_OFF) as *mut u8).write(cnt.wrapping_add(1));
        let watcher = ((esi + WATCHER_OFF) as *const u32).read_unaligned();
        if watcher != 0 {
            let st = ((watcher + STATE_OFF) as *const u32).read_unaligned() & STATE_MASK;
            if st == STATE_WANT {
                let child = ((esi + CHILD_OFF) as *const u32).read_unaligned();
                let r2: u32 = lf_checker_rt::callee_stdcall!(C3, u32, child, 1u32);
                return r2;
            }
        }
        r1
    }
});
