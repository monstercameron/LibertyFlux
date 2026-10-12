// original: 0x00D08F20 squad_count_accepting_members

/// Count live squad members that accept an order.
///
/// Walks the sixteen member slots at `+0x168` of the squad block at
/// `a1 + 0x224`: a null slot, or one whose detached byte at `+0x211` is set,
/// is skipped, and the rest are offered the order (patched thiscall callee).
/// Returns how many accepted (nonzero low byte).
///
/// Original: 0x00D08F20 (cdecl, one stack word). Name proposed: no merged
/// symbol.
lf_checker_rt::export!(cdecl, rw_00d08f20(a1: u32) -> u32 {
    unsafe {
        const OFFER: u32 = 1;
        const SQUAD_OFF: u32 = 0x224;
        const SLOTS: u32 = 0x168;
        const DETACHED: u32 = 0x211;
        let squad = ((a1 + SQUAD_OFF) as *const u32).read_unaligned();
        let mut n = 0u32;
        let mut i = 0u32;
        while i < 16 {
            let m = ((squad + SLOTS + i * 4) as *const u32).read_unaligned();
            if m != 0 && ((m + DETACHED) as *const u8).read() == 0 {
                let list = ((a1 + SQUAD_OFF) as *const u32).read_unaligned();
                if (lf_checker_rt::callee_thiscall!(OFFER, u32, list, m) & 0xFF) != 0 {
                    n += 1;
                }
            }
            i += 1;
        }
        n
    }
});
