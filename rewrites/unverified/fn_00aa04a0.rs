// original: 0x00aa04a0 stream_gc_pass (proposed)

/// Sweep the node list for entries tagged `tag` and recycle the flagged ones.
///
/// The node list hangs off the shared context (global at file address
/// 0x01bb6674, head two levels down at `+0x26c` then `+0x0`; link at node
/// `+0`). Each node is visited at an inner cursor 0x10 bytes past its base;
/// the tag word at cursor `+0x70` must equal `tag` or the node is skipped.
/// A matching node is offered to the pre-pass hook, then classified by its
/// flag word at cursor `+0x254`: bit 5 runs the virtual finaliser at slot
/// `+0x6c` on the owner at cursor `+0x78` (cursor on the stack) and ends
/// the visit; otherwise bit
/// 3 clears the node's link word through the unlinker, drops a counted
/// reference when the tag word is non-zero, and retags the node to `-1`,
/// and bit 4 looks the node up in the index, detaches and releases a hit,
/// and zeroes its key, link and state bytes. Every visit that was not
/// finalised ends by running the virtual closer at slot `+0x68` on the
/// owner with the cursor on the stack. An empty list returns the head
/// container address; otherwise the last hook answer (or `tag` when the
/// last node visited was skipped).
///
/// Original: 0x00aa04a0 (thiscall, one stack word; two virtual calls).
lf_checker_rt::export!(thiscall, rw_00aa04a0(this: u32, tag: u32) -> u32 {
    unsafe {
        const CONTEXT_GLOBAL: u32 = 0x01bb6674;
        const HEAD_OFF: u32 = 0x26c;
        const CURSOR_DELTA: u32 = 0x10;
        const TAG_OFF: u32 = 0x70;
        const FLAGS_OFF: u32 = 0x254;
        const OWNER_OFF: u32 = 0x78;
        const FINALISE_SLOT: u32 = 0x6c;
        const CLOSE_SLOT: u32 = 0x68;
        const RETAGGED: u32 = 0xffff_ffff;
        const PRE_PASS: u32 = 1;
        const FINALISE: u32 = 2;
        const UNLINK: u32 = 3;
        const DROP_REF: u32 = 4;
        const LOOKUP: u32 = 5;
        const DETACH_HIT: u32 = 6;
        const RELEASE_HIT: u32 = 7;
        const CLOSE: u32 = 8;
        let ctx = lf_checker_rt::global::<u32>(CONTEXT_GLOBAL).read_unaligned();
        let head = ((ctx + HEAD_OFF) as *const u32).read_unaligned();
        let mut node = (head as *const u32).read_unaligned();
        if node == 0 {
            return head;
        }
        let mut answer = tag;
        loop {
            let next = (node as *const u32).read_unaligned();
            let cur = node.wrapping_add(CURSOR_DELTA);
            if ((cur + TAG_OFF) as *const u32).read_unaligned() == tag {
                lf_checker_rt::callee_thiscall!(PRE_PASS, u32, this, cur);
                let flags = ((cur + FLAGS_OFF) as *const u32).read_unaligned();
                if flags & (1 << 5) != 0 {
                    let owner = ((cur + OWNER_OFF) as *const u32).read_unaligned();
                    let vtable = (owner as *const u32).read_unaligned();
                    let at = ((vtable + FINALISE_SLOT) as *const u32).read_unaligned();
                    let fin: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(at as usize);
                    answer = fin(owner, cur);
                } else {
                    if flags & (1 << 3) != 0 {
                        answer =
                            lf_checker_rt::callee_thiscall!(UNLINK, u32, this, cur);
                        let tag_now =
                            ((cur + TAG_OFF) as *const u32).read_unaligned();
                        if tag_now != 0 {
                            answer = lf_checker_rt::callee_thiscall!(
                                DROP_REF, u32, tag_now, cur + TAG_OFF
                            );
                        }
                        ((cur + TAG_OFF) as *mut u32).write_unaligned(RETAGGED);
                    }
                    let flags2 = ((cur + FLAGS_OFF) as *const u32).read_unaligned();
                    if flags2 & (1 << 4) != 0 {
                        let hit: u32 =
                            lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, cur);
                        answer = hit;
                        if hit != 0 {
                            let payload =
                                ((hit + 0x0c) as *const u32).read_unaligned();
                            answer = lf_checker_rt::callee_thiscall!(
                                DETACH_HIT, u32, this, payload
                            );
                            ((hit + 0x0c) as *mut u32).write_unaligned(0);
                            ((hit + 8) as *mut u32).write_unaligned(0);
                            ((hit + 0x18) as *mut u8).write(0);
                            answer = lf_checker_rt::callee_thiscall!(
                                RELEASE_HIT, u32, this, hit
                            );
                        }
                    }
                    let owner = ((cur + OWNER_OFF) as *const u32).read_unaligned();
                    let vtable = (owner as *const u32).read_unaligned();
                    let at = ((vtable + CLOSE_SLOT) as *const u32).read_unaligned();
                    let close: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(at as usize);
                    answer = close(owner, cur);
                }
            } else {
                answer = tag;
            }
            node = next;
            if node == 0 {
                break;
            }
        }
        answer
    }
});
