// original: 0x00b50a30 event_singleton (proposed)

/// Return the shared event record, creating it on first use.
///
/// The slot at game address `EVENT_SLOT` holds the record pointer (or
/// null). When null, 76 bytes are allocated (callee 1, cdecl operator new);
/// a failed allocation clears the slot and returns null, otherwise the
/// event constructor (callee 2, thiscall on the fresh block) runs and its
/// result is stored in the slot and returned.
///
/// Original: 0x00b50a30 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00b50a30() -> u32 {
    unsafe {
        const EVENT_SLOT: u32 = 0x01668394;
        const EVENT_SIZE: u32 = 0x4c;
        const OPERATOR_NEW: u32 = 1;
        const CTOR: u32 = 2;
        let slot = lf_checker_rt::global::<u32>(EVENT_SLOT);
        let cur = slot.read_unaligned();
        if cur != 0 {
            return cur;
        }
        let fresh = lf_checker_rt::callee_cdecl!(OPERATOR_NEW, u32, EVENT_SIZE);
        if fresh == 0 {
            slot.write_unaligned(0);
            return 0;
        }
        let built = lf_checker_rt::callee_thiscall!(CTOR, u32, fresh);
        slot.write_unaligned(built);
        built
    }
});
