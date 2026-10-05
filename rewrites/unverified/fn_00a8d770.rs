// original: 0x00a8d770 pool_notify_matching (proposed)

/// Notify the sweep entries, then every matching table node, in order.
///
/// `this` is the pool. When the sweep byte at +0x75 is set and the
/// enable byte at +0x73 is set, entries stream through the next callee
/// (scripted to two entries then done): a non-null entry whose word at
/// +0x44 is not -1 is notified through its slot. (Enable clear with
/// sweep set returns the incoming register, fixed by the proof.)
/// Otherwise the sweep is skipped. Then each bucket below the 16-bit
/// limit at +0xE8 walks its node list from table+bucket*0xA0+8: each
/// node's object resolves through its word at +0x34, and a resolved
/// object whose word at +0x44 is not -1 and whose word at +0x28 selects
/// bit 0x100 is notified through its slot. Returns the bucket limit
/// (reloaded into eax by the loop bottom, discarding notify answers),
/// or the incoming register on the immediate-return path.
///
/// Original: 0x00A8D770 (thiscall, no stack words, vtable callees).
lf_checker_rt::export!(thiscall, rw_00a8d770(this: u32) -> u32 {
    unsafe {
        const CALLEE_NEXT: u32 = 1;
        const CALLEE_NOTIFY: u32 = 2;
        const ENABLE: u32 = 0x73;
        const SWEEP: u32 = 0x75;
        const STATE_A: u32 = 0xfc;
        const STATE_B: u32 = 0x100;
        const ENTRY_GEN: u32 = 0x44;
        const RETIRED: u32 = 0xffff;
        const TABLE: u32 = 0xe4;
        const LIMIT: u32 = 0xe8;
        const BUCKET_SIZE: u32 = 0xa0;
        const BUCKET_HDR: u32 = 8;
        const NODE_NEXT: u32 = 4;
        const OBJ_LINK: u32 = 0x34;
        const OBJ_SEL: u32 = 0x28;
        const SELECT_BIT: u32 = 0x100;
        const SELECT_MASK: u32 = 0x3c0;
        const NOTIFY_SLOT: u32 = 0x44;
        const INCOMING_EAX: u32 = 0x12345678;
        let enabled = ((this + ENABLE) as *const u8).read_unaligned();
        let sweep = ((this + SWEEP) as *const u8).read_unaligned();
        if enabled == 0 && sweep != 0 {
            return INCOMING_EAX;
        }
        if sweep != 0 {
            ((this + STATE_A) as *mut u32).write_unaligned(0);
            ((this + STATE_B) as *mut u32).write_unaligned(0xffffffff);
            let mut slot: u32 = 0;
            let slot_addr = core::ptr::addr_of_mut!(slot) as u32;
            let mut more = lf_checker_rt::callee_thiscall!(
                CALLEE_NEXT,
                u32,
                this,
                slot_addr
            );
            if more as u8 != 0 {
                loop {
                    let entry = slot;
                    if entry != 0 {
                        let gen = ((entry + ENTRY_GEN) as *const u16)
                            .read_unaligned() as u32;
                        if gen != RETIRED {
                            let vtable =
                                (entry as *const u32).read_unaligned();
                            let target =
                                ((vtable + NOTIFY_SLOT) as *const u32)
                                    .read_unaligned();
                            let f: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute(target as usize);
                            f(entry);
                        }
                    }
                    more = lf_checker_rt::callee_thiscall!(
                        CALLEE_NEXT,
                        u32,
                        this,
                        slot_addr
                    );
                    if more as u8 == 0 {
                        break;
                    }
                }
            }
        }
        let table = ((this + TABLE) as *const u32).read_unaligned();
        let limit =
            ((this + LIMIT) as *const u16).read_unaligned() as u32;
        let mut bucket = 0u32;
        while bucket < limit {
            let mut node = (table
                .wrapping_add(bucket.wrapping_mul(BUCKET_SIZE))
                .wrapping_add(BUCKET_HDR)
                as *const u32)
                .read_unaligned();
            while node != 0 {
                let obj = (node as *const u32).read_unaligned();
                node = ((node + NODE_NEXT) as *const u32).read_unaligned();
                let linked =
                    ((obj + OBJ_LINK) as *const u32).read_unaligned();
                if linked == 0 {
                    continue;
                }
                let gen = ((linked + ENTRY_GEN) as *const u16)
                    .read_unaligned() as u32;
                if gen == RETIRED {
                    continue;
                }
                let sel = ((linked + OBJ_SEL) as *const u32).read_unaligned();
                if sel & SELECT_MASK != SELECT_BIT {
                    continue;
                }
                let vtable = (linked as *const u32).read_unaligned();
                let target = ((vtable + NOTIFY_SLOT) as *const u32)
                    .read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(linked);
            }
            bucket += 1;
        }
        // The loop bottom reloads the limit into eax every pass, so the
        // return is the limit (0 when no bucket ran), whatever the
        // notify callees answered.
        limit
    }
});
