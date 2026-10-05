// original: 0x00938790 stream_ready_gate_a (proposed)

/// Decide whether the streaming request may start, through ten gates.
///
/// The request id must match the table's slot unless skipping is
/// allowed; then the mode must exceed 2, the marker must sit below the
/// counter, three block flags must agree, the owner must be set, the
/// poll must be quiet, the clear flag must be down and the scan nonempty.
/// Then the handle is opened: a null handle, a matching stamp word or a
/// zero mode reaches the tail, which answers the state with AL set
/// except for state 0 (flush, AL cleared). A mismatching stamp flushes
/// and answers 0. The two pre-call gates answer the table base with AL
/// cleared. Later 0-answers carry the last call's leftover.
lf_checker_rt::export!(cdecl, rw_00938790(want: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 1;
        const POLL: u32 = 2;
        const SCAN: u32 = 3;
        const OPEN: u32 = 4;
        const FLUSH: u32 = 5;
        const TABLE: u32 = 0x118D804;
        const SLOT: u32 = 0x540;
        const SKIP: u32 = 0x1284654;
        const MODE: u32 = 0x11A4EF4;
        const MARKER: u32 = 0x11A4EFC;
        const BLOCK_A: u32 = 0x18B6ED7;
        const NEED: u32 = 0x11DB27F;
        const BLOCK_B: u32 = 0x118DC40;
        const OWNER: u32 = 0x1160C8C;
        const CLEAR: u32 = 0x11609F6;
        const WANT: u32 = 0x11A4F10;
        const STATE: u32 = 0x11A4EF8;
        const TAG_OFF: u32 = 0x2C;
        const LOW_MASK: u32 = 0xFFFF_FF00;
        let g = |va: u32| -> u32 {
            lf_checker_rt::global::<u32>(va).read_unaligned()
        };
        let gb = |va: u32| -> u8 {
            lf_checker_rt::global::<u8>(va).read()
        };
        // NOTE: the table base is loaded into EAX first, so the two
        // pre-call gates answer it with the low byte cleared.
        let base = g(TABLE);
        let cell = ((base + SLOT) as *const u32).read_unaligned();
        if want != cell && g(SKIP) != 0 {
            return base & LOW_MASK;
        }
        if g(MODE) <= 2 {
            return base & LOW_MASK;
        }
        let e: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32,);
        if g(MARKER) >= e {
            return e & LOW_MASK;
        }
        if gb(BLOCK_A) != 0 {
            return e & LOW_MASK;
        }
        if gb(NEED) == 0 {
            return e & LOW_MASK;
        }
        if gb(BLOCK_B) != 0 {
            return e & LOW_MASK;
        }
        if g(OWNER) == 0 {
            return e & LOW_MASK;
        }
        let q: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
        if (q & 0xFF) != 0 {
            return q & LOW_MASK;
        }
        if gb(CLEAR) != 0 {
            return q & LOW_MASK;
        }
        let s: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32,);
        if (s & 0xFF) == 0 {
            return s & LOW_MASK;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, 0);
        if p != 0 {
            let want_w = g(WANT) & 0xFFFF;
            let have = ((p + TAG_OFF) as *const u16).read_unaligned() as u32;
            if want_w != have && g(MODE) != 0 {
                let r: u32 = lf_checker_rt::callee_cdecl!(FLUSH, u32, 1);
                return r & LOW_MASK;
            }
        }
        // NOTE: tail 1-answers carry the reloaded state's upper bits.
        let v = g(STATE);
        if v == 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(FLUSH, u32, 1);
            return r & LOW_MASK;
        }
        (v & LOW_MASK) | 1
    }
});
