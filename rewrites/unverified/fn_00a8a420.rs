// original: 0x00a8a420 pool_acquire_guarded (proposed)

/// Acquire the guarded slot for the key object, scanning on a miss.
///
/// `this` is the pool and `key` points to the key object. Under the
/// shared lock, the key's word at +0x28 selects the path: kinds 2 and 3
/// (bits 6..9) return 0 at once. Otherwise the find callee runs on the
/// key; a non-null answer is returned. On a miss the node list at
/// +0x18 is walked to its anchor at +8 for the first node whose object
/// has bit 0xA00 in its word at +0x24, that object is released through
/// its slot, and the find callee runs once more. Always unlocks before
/// returning. The proof cycles the kind through all paths; lock, unlock
/// and find frame addresses are uncompared.
///
/// Original: 0x00A8A420 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8a420(this: u32, key: u32) -> u32 {
    unsafe {
        const CALLEE_LOCK: u32 = 1;
        const CALLEE_FIND: u32 = 2;
        const CALLEE_RELEASE: u32 = 3;
        const CALLEE_UNLOCK: u32 = 4;
        const LOCK_ID: u32 = 0x12fb1dc;
        const KEY_KIND: u32 = 0x28;
        const KIND_SHIFT: u32 = 6;
        const KIND_MASK: u32 = 0xf;
        const ANCHOR: u32 = 8;
        const LIST_HEAD: u32 = 0x18;
        const NODE_NEXT: u32 = 4;
        const OBJ_FLAGS: u32 = 0x24;
        const DIRTY_BIT: u32 = 0xa00;
        const RELEASE_SLOT: u32 = 0x44;
        let mut slot: u32 = 0;
        let slot_addr = core::ptr::addr_of_mut!(slot) as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_LOCK,
            u32,
            slot_addr,
            lf_checker_rt::relocated(LOCK_ID)
        );
        let kind_bits = ((key + KEY_KIND) as *const u32).read_unaligned();
        let kind = (kind_bits >> KIND_SHIFT) & KIND_MASK;
        if kind == 2 || kind == 3 {
            lf_checker_rt::callee_thiscall!(CALLEE_UNLOCK, u32, slot_addr);
            return 0;
        }
        let anchor = this.wrapping_add(ANCHOR);
        // The original passes the address of its key slot (a frame
        // pointer, uncompared); the stub never writes it observably.
        let mut key_slot = key;
        let key_addr = core::ptr::addr_of_mut!(key_slot) as u32;
        let found = lf_checker_rt::callee_thiscall!(
            CALLEE_FIND,
            u32,
            anchor,
            key_addr
        );
        if found != 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_UNLOCK, u32, slot_addr);
            return found;
        }
        let mut node =
            ((this + LIST_HEAD) as *const u32).read_unaligned();
        if node != anchor {
            loop {
                let obj = (node as *const u32).read_unaligned();
                let flags =
                    ((obj + OBJ_FLAGS) as *const u32).read_unaligned();
                if flags & DIRTY_BIT != 0 {
                    let vtable =
                        (obj as *const u32).read_unaligned();
                    let target = ((vtable + RELEASE_SLOT) as *const u32)
                        .read_unaligned();
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(target as usize);
                    f(obj);
                    break;
                }
                node =
                    ((node + NODE_NEXT) as *const u32).read_unaligned();
                if node == anchor {
                    break;
                }
            }
        }
        let found2 = lf_checker_rt::callee_thiscall!(
            CALLEE_FIND,
            u32,
            anchor,
            key_addr
        );
        lf_checker_rt::callee_thiscall!(CALLEE_UNLOCK, u32, slot_addr);
        found2
    }
});
