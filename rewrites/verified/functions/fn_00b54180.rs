// original: 0x00B54180 crmt_forward_after_scan (proposed)

/// Scan the node chain for a ready entry, then forward all arguments.
///
/// When bit 8 of `a1` is set, walks the chain rooted at `this` + 0x1A28
/// (next at +0x8C): the first node whose word at +0x48 equals 1, whose
/// dword at +0x44 is non-zero and whose dword at +0x8 has bit 8 set is
/// picked (all equality tests on unsigned values). The forwarder (callee
/// 1, thiscall on `this`) then runs with the picked node plus 4, or zero
/// when the gate was off or nothing matched, followed by the nine
/// incoming words in order. Returns the forwarder's answer.
///
/// Original: 0x00B54180 (thiscall, nine stack words).
lf_checker_rt::export!(thiscall, rw_00b54180(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x1a28;
        const KIND: u32 = 0x48;
        const READY: u32 = 0x44;
        const FLAGS: u32 = 0x8;
        const NEXT: u32 = 0x8c;
        const FWD: u32 = 1;
        let mut found = 0u32;
        if ((a1 >> 8) & 1) != 0 {
            let mut node = ((this + HEAD) as *const u32).read_unaligned();
            while node != 0 {
                let is_one = ((node + KIND) as *const u16).read_unaligned() == 1;
                let next = ((node + NEXT) as *const u32).read_unaligned();
                if is_one {
                    let ready = ((node + READY) as *const u32).read_unaligned();
                    if ready != 0 {
                        let flags = ((node + FLAGS) as *const u32).read_unaligned();
                        if ((flags >> 8) & 1) != 0 {
                            found = node.wrapping_add(4);
                            break;
                        }
                    }
                }
                node = next;
            }
        }
        lf_checker_rt::callee_thiscall!(
            FWD, u32, this, found, a0, a1, a2, a3, a4, a5, a6, a7, a8
        )
    }
});
