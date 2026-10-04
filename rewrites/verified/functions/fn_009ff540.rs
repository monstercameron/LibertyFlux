// original: 0x009FF540 frag_watch_list_sweep (proposed)

/// Sweep a frag watch list, refreshing stale watched objects.
///
/// Walks the singly-linked list at `[arg]` (each node a watched object
/// followed by the next node). For a non-null object whose stamp (`+0x3c`)
/// differs from the global stamp word, the stamp is updated and, when the
/// object is idle (`+0x38` zero), of kind 1 or 8 (bits 6..9 of `+0x28`)
/// and marked (`+0x28` bit 27), its two virtual hooks run in order; the
/// second runs only if the first left the object non-idle. A hooked object
/// is then offered to the matcher callee with a pointer to a slot holding
/// the current object (the original spills it to its incoming argument
/// slot, which a rewrite cannot address, so the stack check is off for this
/// contract), and a nonzero match runs the match callee on (`obj`, match).
/// Returns nothing.
///
/// Original: 0x009FF540 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FF540(arg: u32) -> u32 {
    unsafe {
        const STAMP: u32 = 0x011A8908;
        const MATCHER_OBJ: u32 = 0x012BCCD0;
        const OBJ_OFF: u32 = 0;
        const NEXT_OFF: u32 = 4;
        const STAMP_OFF: u32 = 0x3C;
        const IDLE_OFF: u32 = 0x38;
        const FLAGS_OFF: u32 = 0x28;
        const MARK_BIT: u32 = 0x0800_0000;
        const HOOK0_SLOT: usize = 0xA8;
        const HOOK1_SLOT: usize = 0xAC;
        let stamp = (lf_checker_rt::relocated(STAMP) as *const u16).read_unaligned() as u32;
        let mut node = (arg as *const u32).read_unaligned();
        if node == 0 {
            return 0;
        }
        loop {
            let obj = ((node + OBJ_OFF) as *const u32).read_unaligned();
            let next = ((node + NEXT_OFF) as *const u32).read_unaligned();
            if obj != 0 {
                if ((obj + STAMP_OFF) as *const u32).read_unaligned() != stamp {
                    ((obj + STAMP_OFF) as *mut u32).write_unaligned(stamp);
                    if ((obj + IDLE_OFF) as *const u32).read_unaligned() == 0 {
                        let flags = ((obj + FLAGS_OFF) as *const u32).read_unaligned();
                        let kind = (flags >> 6) & 0xF;
                        if (kind == 1 || kind == 8) && flags & MARK_BIT != 0 {
                            let vt = (obj as *const u32).read_unaligned();
                            let h0: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute((((vt as *const u8).add(HOOK0_SLOT))
                                    as *const u32)
                                    .read_unaligned()
                                    as usize);
                            h0(obj);
                            if ((obj + IDLE_OFF) as *const u32).read_unaligned() != 0 {
                                let vt = (obj as *const u32).read_unaligned();
                                let h1: extern "thiscall" fn(u32) -> u32 =
                                    core::mem::transmute((((vt as *const u8).add(HOOK1_SLOT))
                                        as *const u32)
                                        .read_unaligned()
                                        as usize);
                                h1(obj);
                                let mut slot = obj;
                                let r = lf_checker_rt::callee_thiscall!(3, u32,
                                    lf_checker_rt::relocated(MATCHER_OBJ),
                                    &mut slot as *mut u32 as u32);
                                if r != 0 {
                                    lf_checker_rt::callee_thiscall!(4, u32, obj, r);
                                }
                            }
                        }
                    }
                }
            }
            if next == 0 {
                break;
            }
            node = next;
        }
        0
    }
});
