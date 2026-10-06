// original: 0x00B54F60 crmtManagerPriority::vf6

/// Notify the channel at `idx`, then insert a zero there, shifting up.
///
/// The worker at callee 1 runs first with (`this`, `idx`); its answer is
/// ignored. The 32-entry table at `this` + 0x828 is then shifted one slot
/// towards higher indices starting above `idx` (entries `idx` through 30
/// move up one place, the old entry 31 is dropped) and entry `idx` is set
/// to zero. When `idx` is 31 or more (compared unsigned) no shift happens
/// and only the store runs, which writes past the table for `idx` above
/// 31. Returns the notifier's answer when no shift ran, else the address
/// one past the last shifted entry.
///
/// Original: 0x00B54F60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54f60(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x828;
        const ENTRIES: u32 = 32;
        const TOP: u32 = 0x8a4;
        const NOTIFY: u32 = 1;
        let out: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, idx);
        if idx >= ENTRIES - 1 {
            ((this + TABLE).wrapping_add(idx.wrapping_mul(4)) as *mut u32)
                .write_unaligned(0);
            return out;
        }
        let mut slot = this + TOP;
        let mut left = (ENTRIES - 1).wrapping_sub(idx);
        while left != 0 {
            let v = ((slot - 4) as *const u32).read_unaligned();
            (slot as *mut u32).write_unaligned(v);
            slot -= 4;
            left -= 1;
        }
        ((this + TABLE).wrapping_add(idx.wrapping_mul(4)) as *mut u32)
            .write_unaligned(0);
        slot
    }
});
