// original: 0x009DBFE0 pool_slot_alloc_0x80 (proposed)

/// Take the next slot of a typed object pool and register it in the shared table.
///
/// The pool's element count lives in a global (0x103ad8c) and its element
/// base in the next (0x103ad90). Slot `count` sits at `count * 128 +
/// base`; the count is bumped past it. The slot's vtable slot-1 initialiser
/// runs with the slot as `this`, the stack argument is hashed into the
/// slot's word at `+0x3C`, and the slot is published in the shared object
/// table under a second global index (0x12b415c). A registry flag byte
/// (0x103ae84) is cleared, a registry node is allocated holding the
/// (hash, index) pair, the index is bumped, and the slot is returned.
///
/// Original: 0x009DBFE0 (cdecl, one stack argument; three outgoing calls:
/// the slot initialiser through the slot vtable, the argument hash, and the
/// registry-node allocator).
lf_checker_rt::export!(cdecl, rw_009DBFE0(arg: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x80;
        const COUNT_VA: u32 = 0x0103AD8C;
        const BASE_VA: u32 = 0x0103AD90;
        const INDEX_VA: u32 = 0x012B415C;
        const TABLE_VA: u32 = 0x01295CD8;
        const FLAG_VA: u32 = 0x0103AE84;
        const REGISTRY_VA: u32 = 0x0103AE88;
        const SLOT_HASH_OFF: u32 = 0x3C;
        const HASH_CALLEE: u32 = 2;
        const NODE_CALLEE: u32 = 3;

        let count = (lf_checker_rt::global::<u32>(COUNT_VA) as *const u32).read_unaligned();
        let base = (lf_checker_rt::global::<u32>(BASE_VA) as *const u32).read_unaligned();
        let slot = count.wrapping_shl(7).wrapping_add(base);
        lf_checker_rt::global::<u32>(COUNT_VA).write_unaligned(count.wrapping_add(1));
        // Slot initialiser through the slot's own vtable, exactly like the original.
        let vtable = (slot as *const u32).read_unaligned();
        let init: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (((vtable as *const u32).wrapping_add(1)).read_unaligned()) as usize,
        );
        init(slot);
        let hash: u32 = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, arg);
        ((slot as *mut u8).wrapping_add(SLOT_HASH_OFF) as *mut u32).write_unaligned(hash);
        let index = (lf_checker_rt::global::<u32>(INDEX_VA) as *const u32).read_unaligned();
        (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(index as usize)).write_unaligned(slot);
        let stored = ((slot as *const u8).wrapping_add(SLOT_HASH_OFF) as *const u32).read_unaligned();
        lf_checker_rt::global::<u8>(FLAG_VA).write(0);
        let node: u32 = lf_checker_rt::callee_thiscall!(
            NODE_CALLEE, u32, lf_checker_rt::relocated(REGISTRY_VA), 0x10u32
        );
        (node as *mut u32).write_unaligned(stored);
        let index_now =
            (lf_checker_rt::global::<u32>(INDEX_VA) as *const u32).read_unaligned();
        ((node as *mut u32).wrapping_add(1)).write_unaligned(index_now);
        lf_checker_rt::global::<u32>(INDEX_VA).write_unaligned(index.wrapping_add(1));
        slot
    }
});
