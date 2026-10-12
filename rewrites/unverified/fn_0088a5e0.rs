// original: 0x0088A5E0 stream_resource_next_slot_generation (proposed)

/// Take the next generation number for a resource slot.
///
/// `this` points to the resource manager with table base at `+0xe8`. Locks
/// the group's guard word at `group * 0x6f40 + base + 0x6f1c` (patched
/// callee), reads the group's current sequence byte at `+0x6f18`,
/// increments it in place, unlocks (patched callee), and returns
/// `(group << 8 | old_byte) + 1`.
///
/// Original: 0x0088A5E0 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_0088a5e0(this: u32, group: u32) -> u32 {
    unsafe {
        const GUARD_LOCK: u32 = 1;
        const GUARD_UNLOCK: u32 = 2;
        const TABLE_BASE: u32 = 0xe8;
        const GUARD_OFF: u32 = 0x6f1c;
        const SEQ_OFF: u32 = 0x6f18;
        const GROUP_STRIDE: u32 = 0x6f40;

        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let guard = base
            .wrapping_add(GUARD_OFF)
            .wrapping_add(group.wrapping_mul(GROUP_STRIDE));
        lf_checker_rt::callee_thiscall!(GUARD_LOCK, u32, guard);
        let cell = base
            .wrapping_add(group.wrapping_mul(GROUP_STRIDE))
            .wrapping_add(SEQ_OFF);
        let old = (cell as *const u8).read();
        (cell as *mut u8).write(old.wrapping_add(1));
        lf_checker_rt::callee_thiscall!(GUARD_UNLOCK, u32, guard);
        group.wrapping_shl(8).wrapping_add(old as u32).wrapping_add(1)
    }
});
