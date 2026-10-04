// original: 0x00968130 timers_all_ready
/// Report whether every active timer entry in the four groups is ready.
///
/// Four groups of 0x60-byte entries start at `this + 0x60` (3 entries),
/// `this + 0x180` (3), `this + 0x2A0` (4) and `this + 0x420` (2). An
/// entry is active when the flag dword 0x10 bytes below its base is
/// nonzero; an active entry is ready when bit 1 of its first byte is set.
/// Returns 1 in `al` when no active entry is unready, else 0. The flag
/// byte is read only for active entries. Upper `eax` is left over from
/// entry state, so the return channel is `al` only.
///
/// Original: 0x00968130 (thiscall, no stack words).

export!(thiscall, rw_00968130(this: u32) -> u32 {
    unsafe {
        const READY_BIT: u8 = 0x02;
        const GROUPS: [(u32, u32); 4] = [(0x60, 3), (0x180, 3), (0x2A0, 4), (0x420, 2)];
        for (base, count) in GROUPS {
            for i in 0..count {
                let entry = this.wrapping_add(base).wrapping_add(i * 0x60);
                if (entry.wrapping_sub(0x10) as *const u32).read_unaligned() != 0
                    && (entry as *const u8).read() & READY_BIT == 0
                {
                    return 0;
                }
            }
        }
        1
    }
});
