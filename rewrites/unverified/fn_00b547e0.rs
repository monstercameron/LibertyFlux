// original: 0x00B547E0 crmt_slot_dispatch (proposed)

/// Find `key` in the 32-entry table and dispatch to that slot's handler.
///
/// Scans the table at `this` + 0x30C for `key` (the second stack word;
/// the first is unread) with a signed less-than against 32 that never
/// sees a negative value. When no entry matches, returns 32. On a match
/// at index `i`, loads the slot object from `this` + 0x78C + `i` * 4,
/// adds 0x10 to get the target, and calls the function pointer at target
/// + 0xC with (`target`, `i`), returning its answer.
///
/// Original: 0x00B547E0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00b547e0(this: u32, _unused: u32, key: u32) -> u32 {
    unsafe {
        const SCAN_BASE: u32 = 0x30c;
        const PTR_BASE: u32 = 0x78c;
        const TARGET_OFF: u32 = 0x10;
        const HANDLER_OFF: u32 = 0xc;
        const COUNT: u32 = 0x20;
        let mut idx = 0u32;
        let mut slot = this + SCAN_BASE;
        loop {
            if (slot as *const u32).read_unaligned() == key {
                break;
            }
            idx += 1;
            slot += 4;
            if idx >= COUNT {
                return COUNT;
            }
        }
        let obj = ((this + PTR_BASE).wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let target = obj.wrapping_add(TARGET_OFF);
        let handler = ((target + HANDLER_OFF) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(handler as usize);
        f(target, idx)
    }
});
