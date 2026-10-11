// original: 0x0060A040 physics_restore_joint_cache

/// Restore the twelve defined cached words from `source` into the matching
/// `this` fields. Each three-word group is followed by an untouched dword gap.
/// The machine return carries the final copied word. This is a 32-bit
/// thiscall with the source pointer on the stack.
lf_checker_rt::export!(thiscall, rw_0060A040(this: u32, source: u32) -> u32 {
    unsafe {
        const OFFSETS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        const DESTINATION: u32 = 0xd0;
        let mut last = 0;
        for offset in OFFSETS {
            last = ((source + offset) as *const u32).read_unaligned();
            ((this + DESTINATION + offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
