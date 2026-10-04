// original: 0x00A9B2E0 files-memory entry probe (unnamed in symbols)
/// Probe one table entry: validate it, then count flagged list nodes.
///
/// `this` is the entry and `arg` the probing context. After validating
/// the entry's owner chain, a default-threshold reading marks the entry.
/// The owner is then asked for its current source object: when one
/// answers and every per-slot check passes, the owner is released and
/// the probe reports zero. Otherwise the entry's node list is walked
/// (each node is offered to a matcher first) and the probe reports how
/// many nodes carry the byte flag plus how many carry a positive count;
/// a nonzero byte-flag total also refreshes the owner and runs the
/// tail notifier.
lf_checker_rt::export!(thiscall, rb110_fn2(this: u32, arg: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_SOURCE: u32 = 1; // source resolve (thiscall/0)
    const CAL_SLOT: u32 = 2; // per-slot check (thiscall/1, byte result)
    const VT_QUERY_SLOT: u32 = 0xA0; // vtable slot answering the source
    const VT_RELEASE_SLOT: u32 = 0x98; // vtable slot releasing the owner

    // Globals (file VAs; resolved through the worker's image base).
    const K_DEFAULT_VA: u32 = 0x00FE8D18; // default threshold (FLT_MAX)

    /// Call a planted vtable slot exactly like the original. Both sides
    /// land on the same recorder stub.
    #[inline(always)]
    unsafe fn vcall0(object: u32, slot: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object)
    }

    /// Walk the entry's node list, offering each node to the matcher,
    /// and report flagged-node counts. Shared by both failure paths.
    #[inline(always)]
    unsafe fn walk_nodes(this: u32, arg: u32, owner: u32) -> u32 {
        const CAL_MATCH: u32 = 3;
        const CAL_REFRESH: u32 = 4;
        const CAL_NOTIFY: u32 = 5;
        const VT_QUERY_SLOT: u32 = 0xA0;
        // SAFETY: all addresses come from the validated entry graph or
        // checker-scripted answers, exactly as on the original side.
        unsafe {
            let mut byte_flags: u32 = 0;
            let mut pos_counts: u32 = 0;
            let mut node = *((this.wrapping_add(0x10)) as *const u32);
            while node != 0 {
                lf_checker_rt::callee_thiscall!(CAL_MATCH, u32, node, this, arg);
                if *((node.wrapping_add(0x14)) as *const u8) != 0 {
                    byte_flags = byte_flags.wrapping_add(1);
                }
                if *((node.wrapping_add(0x18)) as *const i32) > 0 {
                    pos_counts = pos_counts.wrapping_add(1);
                }
                node = *(node as *const u32);
            }
            if byte_flags > 0 {
                // A nonzero flag total refreshes the owner first.
                let r = vcall0(owner, VT_QUERY_SLOT);
                if r != 0 {
                    let r2 = vcall0(owner, VT_QUERY_SLOT);
                    let index = *((this.wrapping_add(8)) as *const u32);
                    lf_checker_rt::callee_thiscall!(CAL_REFRESH, u32, r2, index);
                }
                lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this);
            }
            byte_flags.wrapping_add(pos_counts)
        }
    }

    unsafe {
        // Owner chain: entry -> owner -> inner, inner word nonzero.
        let owner = *((this.wrapping_add(0x68)) as *const u32);
        if owner == 0 {
            return 0;
        }
        let inner = *((owner.wrapping_add(0x34)) as *const u32);
        if inner == 0 {
            return 0;
        }
        if *(inner as *const u32) == 0 {
            return 0;
        }
        // A default-threshold reading marks the entry (exact float equality).
        let def = *(lf_checker_rt::global::<f32>(K_DEFAULT_VA));
        let x = *((arg.wrapping_add(0x10)) as *const f32);
        if x == def {
            *((this.wrapping_add(0x86)) as *mut u8) = 1;
        }
        // Ask the owner for its source object.
        let src = vcall0(owner, VT_QUERY_SLOT);
        if src == 0 {
            return walk_nodes(this, arg, owner);
        }
        // Per-slot checks over the resolved table row.
        let index = *((this.wrapping_add(8)) as *const u32);
        let tbl = lf_checker_rt::callee_thiscall!(CAL_SOURCE, u32, src);
        let slots = *((tbl.wrapping_add(0x1F2)) as *const u8);
        let tab = *((tbl.wrapping_add(0xD4)) as *const u32);
        let row = *((tab.wrapping_add(index.wrapping_mul(4))) as *const u32);
        let skip = *((row.wrapping_add(0x0C)) as *const u8);
        let mut i: u32 = 0;
        while i < slots as u32 {
            if i != skip as u32 {
                // The original tests only the low byte of the answer.
                let ok = lf_checker_rt::callee_thiscall!(CAL_SLOT, u32, src, i);
                if (ok & 0xFF) == 0 {
                    return walk_nodes(this, arg, owner);
                }
            }
            i = i.wrapping_add(1);
        }
        vcall0(owner, VT_RELEASE_SLOT);
        0
    }
});
