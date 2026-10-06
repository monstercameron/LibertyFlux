// original: 0x008726D0 node_unlink_release
/// Unlink a node from its parent graph, then release one reference.
///
/// When the parent word at `this + 0x0C` is set, the current head is
/// fetched through virtual slot `0x1C` of the parent's table (compared
/// for exact equality with `this`, and for null: no signedness). A head
/// equal to `this` moves `this`'s `+0x10` link into the parent's `+0x1C`
/// slot; any other non-null head starts a walk down `+0x10` links that
/// splices `this` out where found. Both link words are cleared either
/// way. With no parent, the back-link through `+8` is cleared instead,
/// and a mismatched back-link returns the owner pointer at once.
/// Otherwise the 16-bit reference count at `+4` is decremented: a
/// surviving count returns `0xFFFF`, while a count reaching zero deletes
/// through the owner hook at `+0x18` (direct helper, stdcall) or virtual
/// slot 0 with argument 1, returning the deleter's answer.
///
/// Original: 0x008726D0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008726D0(this: u32) -> u32 {
    unsafe {
        const PARENT_OFF: u32 = 0x0C;
        const LINK_OFF: u32 = 0x10;
        const HEAD_SLOT: u32 = 0x1C;
        const OWNER_OFF: u32 = 8;
        const HOOK_OFF: u32 = 0x18;
        const REF_OFF: u32 = 4;
        const HOOK_CALLEE: u32 = 2;
        let parent = ((this + PARENT_OFF) as *const u32).read_unaligned();
        if parent != 0 {
            let vt = (parent as *const u32).read_unaligned();
            let target = ((vt + HEAD_SLOT) as *const u32).read_unaligned();
            let fetch: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let head = fetch(parent);
            if head == this {
                let next = ((this + LINK_OFF) as *const u32).read_unaligned();
                ((parent + HEAD_SLOT) as *mut u32).write_unaligned(next);
                ((this + LINK_OFF) as *mut u32).write_unaligned(0);
                ((this + PARENT_OFF) as *mut u32).write_unaligned(0);
            } else if head != 0 {
                let mut cursor = head;
                loop {
                    let cand = ((cursor + LINK_OFF) as *const u32).read_unaligned();
                    if cand == this {
                        let next =
                            ((this + LINK_OFF) as *const u32).read_unaligned();
                        ((cursor + LINK_OFF) as *mut u32).write_unaligned(next);
                        ((this + LINK_OFF) as *mut u32).write_unaligned(0);
                        ((this + PARENT_OFF) as *mut u32).write_unaligned(0);
                        break;
                    }
                    cursor = cand;
                    if cursor == 0 {
                        ((this + LINK_OFF) as *mut u32).write_unaligned(cand);
                        ((this + PARENT_OFF) as *mut u32).write_unaligned(cand);
                        break;
                    }
                }
            } else {
                ((this + LINK_OFF) as *mut u32).write_unaligned(0);
                ((this + PARENT_OFF) as *mut u32).write_unaligned(0);
            }
        } else {
            let owner = ((this + OWNER_OFF) as *const u32).read_unaligned();
            let back = ((owner + OWNER_OFF) as *const u32).read_unaligned();
            if back != this {
                return owner;
            }
            ((owner + OWNER_OFF) as *mut u32).write_unaligned(0);
        }
        let refs = ((this + REF_OFF) as *const u16).read_unaligned();
        let left = refs.wrapping_add(0xFFFF);
        ((this + REF_OFF) as *mut u16).write_unaligned(left);
        if left != 0 {
            return 0xFFFF;
        }
        let hook = ((this + HOOK_OFF) as *const u32).read_unaligned();
        if hook != 0 {
            lf_checker_rt::callee_stdcall!(HOOK_CALLEE, u32, this)
        } else {
            let vt = (this as *const u32).read_unaligned();
            let target = (vt as *const u32).read_unaligned();
            let delete: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            delete(this, 1)
        }
    }
});
