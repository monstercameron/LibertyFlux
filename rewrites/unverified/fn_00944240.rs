// original: 0x00944240 streaming_cache_update (proposed)

/// Refresh the streaming cache entry for the current lane, then record it.
///
/// The lane byte at `this + 0x1917` selects a 0xbd0-byte lane record. When
/// the record's kind byte at `+0xbc0` reads 3 and the global pair gate and
/// id gate both agree, a set marker in the global marker table (indexed by
/// the record id at `+0xbc4`) is cleared. Then the entry is resolved
/// through the list lookup (thiscall, one stack argument: the kind byte);
/// a null entry ends the call with 0, and kind 2 with a set hold flag at
/// `+0x1921` ends it with the entry. Otherwise the resolved 10-slot ring
/// (index byte at `+0xf`, slots at `+0x11`) is checked: when the slot
/// selected by `(index + 9) % 10` already holds the lane's current value
/// that slot number is returned, else the value is stored into the indexed
/// slot, the index advanced modulo 10 and the new quotient returned.
///
/// Original: 0x00944240 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00944240(this: u32) -> u32 {
    unsafe {
        const LANE: u32 = 0x1917;
        const LANE_ID: u32 = 0x1918;
        const HOLD: u32 = 0x1921;
        const ALT: u32 = 0x1930;
        const STRIDE: u32 = 0xBD0;
        const KIND: u32 = 0xBC0;
        const REC_ID: u32 = 0xBC4;
        const VALUE_PTR: u32 = 0x960;
        const GATE_A: u32 = 0x0128463C;
        const GATE_B: u32 = 0x01284640;
        const GATE_ID: u32 = 0x01284644;
        const MARKERS: u32 = 0x011D7520;
        const RING_INDEX: u32 = 0xF;
        const RING_SLOTS: u32 = 0x11;
        const RING: u32 = 10;
        const CALLEE: u32 = 1;
        let lane = ((this + LANE) as *const u8).read() as u32;
        let rec = this + lane.wrapping_mul(STRIDE);
        let kind = ((rec + KIND) as *const u8).read() as u32;
        if kind == 3 {
            let gate_a = lf_checker_rt::global::<u32>(GATE_A).read();
            let gate_b = lf_checker_rt::global::<u32>(GATE_B).read();
            if gate_a == 2 || gate_b == 2 {
                let want = lf_checker_rt::global::<u32>(GATE_ID).read();
                let have = ((this + LANE_ID) as *const u8).read() as u32;
                if want == have {
                    let id = ((rec + REC_ID) as *const u32).read_unaligned();
                    let markers = lf_checker_rt::relocated(MARKERS);
                    if ((markers + id) as *const u8).read() == 3 {
                        // The original faults on an id of 0x100 or more
                        // (out of the marker table); inputs keep it below.
                        ((markers + id) as *mut u8).write(0);
                    }
                }
            }
        }
        let entry: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, this, kind);
        if entry == 0 {
            return 0;
        }
        if kind == 2 && ((this + HOLD) as *const u8).read() != 0 {
            return entry;
        }
        let at = ((entry + RING_INDEX) as *const u8).read() as u32;
        let slot = (at + 9) % RING;
        let lane2 = ((this + LANE) as *const u8).read() as u32;
        let rec2 = this + lane2.wrapping_mul(STRIDE);
        let mut value = ((rec2 + VALUE_PTR) as *const u32).read_unaligned();
        if value != 0 {
            value = ((value + 4) as *const u32).read_unaligned();
        }
        if ((this + ALT) as *const u8).read() != 0 && kind == 2 {
            value = ((rec2 + REC_ID) as *const u32).read_unaligned();
        }
        if ((entry + slot * 4 + RING_SLOTS) as *const u32).read_unaligned() == value {
            return slot;
        }
        ((entry + at * 4 + RING_SLOTS) as *mut u32).write_unaligned(value);
        let advanced = (((entry + RING_INDEX) as *const u8).read() as u32).wrapping_add(1);
        ((entry + RING_INDEX) as *mut u8).write((advanced % RING) as u8);
        advanced / RING
    }
});
