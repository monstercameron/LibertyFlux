// original: 0x00a3ad40 vehicle_slot_release (proposed)

/// Release a vehicle table slot and clear the record it pointed at.
///
/// Follows four links (`slot[0]`, `+0x80`, `+0x34`, `+0x4`) to a target
/// record, clears its byte at `+0xcd` and its word at `+0xd4`, marks the
/// second record's `+0x84` as "no index" (-1), then clears the slot itself
/// (`[0] = 0`, `[4] = -1`). Cdecl/1, returns the second record pointer.
lf_checker_rt::export!(cdecl, rw_00a3ad40(slot: u32) -> u32 {
    unsafe {
        const REC2: u32 = 0x80;
        const REC3: u32 = 0x34;
        const TARGET: u32 = 0x04;
        const CLEAR_B: u32 = 0xCD;
        const CLEAR_W: u32 = 0xD4;
        const NO_INDEX_SLOT: u32 = 0x84;
        const NO_INDEX: u32 = 0xFFFF_FFFF;
        let r1 = core::ptr::read_unaligned(slot as *const u32);
        let r2 = core::ptr::read_unaligned((r1 + REC2) as *const u32);
        let r3 = core::ptr::read_unaligned((r2 + REC3) as *const u32);
        let tgt = core::ptr::read_unaligned((r3 + TARGET) as *const u32);
        core::ptr::write((tgt + CLEAR_B) as *mut u8, 0);
        core::ptr::write_unaligned((tgt + CLEAR_W) as *mut u32, 0);
        let r1b = core::ptr::read_unaligned(slot as *const u32);
        core::ptr::write_unaligned((r1b + NO_INDEX_SLOT) as *mut u32, NO_INDEX);
        core::ptr::write_unaligned(slot as *mut u32, 0);
        core::ptr::write_unaligned((slot + 4) as *mut u32, NO_INDEX);
        r1b
    }
});
