// original: 0x0087b990 rage::crmtNodeCapture::vf4
/// Gate, probe, then match-or-dispatch through the partner's table.
///
/// Takes the node pointer in ECX and the partner object on the stack.
/// Opens the node's gate entry with the node: a null answer selects the
/// tag path, which reads the tag word beside the node header and dispatches
/// through the partner's table exactly like the frame sibling (release
/// entry on a clear tag, forward entry carrying the tag otherwise). A live
/// gate answer with a null member link returns the gate answer. Otherwise
/// the partner's probe entry runs with the partner and a constant word;
/// the member's stamp is then compared against the probe answer's stamp and
/// the matching finisher runs on equality, the other finisher on a clear
/// or differing stamp. Each finisher takes a frame slot holding the member
/// plus the probe answer and two scratch words the comparison skips.
export!(thiscall, rw_0087b990(this: u32, partner: u32) -> u32 {
    unsafe {
        const GATE_SLOT: u32 = 0x1C;
        const MEMBER_OFF: u32 = 0x20;
        const STAMP_OFF: u32 = 8;
        const FORWARD_SLOT: u32 = 0x28;
        const RELEASE_SLOT: u32 = 0x48;
        const PROBE_SLOT: u32 = 0x5C;
        const PROBE_WORD: u32 = 1;
        let table_this = *(this as *const u32);
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((table_this.wrapping_add(GATE_SLOT)) as *const u32) as usize);
        let opened = gate(this);
        if opened == 0 {
            let tag = *((this.wrapping_add(MEMBER_OFF)) as *const u32);
            let table = *(partner as *const u32);
            if tag == 0 {
                let release: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(*((table.wrapping_add(RELEASE_SLOT)) as *const u32) as usize);
                return release(partner);
            }
            let forward: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*((table.wrapping_add(FORWARD_SLOT)) as *const u32) as usize);
            return forward(partner, tag);
        }
        let member = *((this.wrapping_add(MEMBER_OFF)) as *const u32);
        if member == 0 {
            return opened;
        }
        let table = *(partner as *const u32);
        let probe: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((table.wrapping_add(PROBE_SLOT)) as *const u32) as usize);
        let found = probe(partner, PROBE_WORD);
        let stamp = *((member.wrapping_add(STAMP_OFF)) as *const u32);
        let mut slot = member;
        let frame = &mut slot as *mut u32 as u32;
        if stamp == 0 || stamp != *((found.wrapping_add(STAMP_OFF)) as *const u32) {
            return callee_thiscall!(4, u32, frame, found, 0, 0);
        }
        callee_thiscall!(3, u32, frame, found, 0, 0)
    }
});
