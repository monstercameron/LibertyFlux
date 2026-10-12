// original: 0x0088A2C0 stream_resource_slot_release (proposed)

/// Release one chunk bit of a resource group under its lock.
///
/// `this` points to the resource manager with table base at `+0xe8`. Locks
/// through a frame-resident lock object (patched callee; the frame address
/// differs per side and is skipped by the proof), clears bit `bit % 32` of
/// the bitmap word at `group * 0x6f40 + base + (bit / 32) * 4 + 0x6f00`,
/// increments the group's free count at `+0x6f0e`, unlocks (patched callee),
/// and returns the inverted one-hot mask (the original's leftover eax).
///
/// Original: 0x0088A2C0 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_0088a2c0(this: u32, group: u32, bit: u32) -> u32 {
    unsafe {
        const GROUP_LOCK: u32 = 1;
        const GROUP_UNLOCK: u32 = 2;
        const TABLE_BASE: u32 = 0xe8;
        const GUARD_OFF: u32 = 0x6f1c;
        const BITMAP_OFF: u32 = 0x6f00;
        const FREE_OFF: u32 = 0x6f0e;
        const GROUP_STRIDE: u32 = 0x6f40;

        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let gbase = group.wrapping_mul(GROUP_STRIDE).wrapping_add(base);
        let mut lock: [u32; 2] = [0, 0];
        lf_checker_rt::callee_thiscall!(
            GROUP_LOCK,
            u32,
            core::ptr::addr_of_mut!(lock) as u32,
            gbase.wrapping_add(GUARD_OFF)
        );
        let mask = !(1u32 << (bit & 31));
        let cell = gbase
            .wrapping_add((bit >> 5).wrapping_mul(4))
            .wrapping_add(BITMAP_OFF);
        let old = (cell as *const u32).read_unaligned();
        (cell as *mut u32).write_unaligned(old & mask);
        let free = (gbase.wrapping_add(FREE_OFF)) as *mut u8;
        free.write(free.read().wrapping_add(1));
        lf_checker_rt::callee_thiscall!(GROUP_UNLOCK, u32, core::ptr::addr_of_mut!(lock) as u32);
        mask
    }
});
