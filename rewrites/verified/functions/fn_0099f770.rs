// original: 0x0099F770 audio_resolve_voice_pair (proposed)

/// Resolve the voice/event pair from the first live source slot.
///
/// Slot A at `this`+0x60 is tried first; only when it is null is slot B at
/// `this`+0x64 tried, and with both null the result is 0. For a live source
/// the result is 1 unless resolution completes: null argument words, a kind
/// byte at +0x3B other than 4, or a zero answer from the voice lookup
/// (callee 1, thiscall/0) each return 1 at once. Otherwise the voice handle
/// from a second lookup feeds the event query (callee 2, thiscall/1 of a
/// zero word), whose answer lands in the second argument word, and the
/// source query (callee 3, thiscall/1) runs with a pointer to a scratch
/// pair holding the first argument word with its low byte cleared beside
/// the second; its answer lands in the first argument word. The original
/// builds that scratch pair over its own incoming argument slots, so the
/// contract disables the stack check and compares the two pointed-to words
/// at call time instead; the rewrite passes an equivalent local pair. A
/// completed resolution returns the source answer with its low byte set to
/// 1, as the original's `(an instruction of the original)` does. Thiscall with two stack words,
/// callee pops 8.
lf_checker_rt::export!(thiscall, rw_0099F770(this: u32, out_a: u32, out_b: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x60;
        const SLOT_B: u32 = 0x64;
        const KIND: u32 = 0x3B;
        const VOICE_KIND: u8 = 4;
        const LOOKUP_CALLEE: u32 = 1;
        const EVENT_CALLEE: u32 = 2;
        const SOURCE_CALLEE: u32 = 3;
        let mut src = ((this.wrapping_add(SLOT_A)) as *const u32).read_unaligned();
        if src == 0 {
            src = ((this.wrapping_add(SLOT_B)) as *const u32).read_unaligned();
            if src == 0 {
                return 0;
            }
        }
        if out_a == 0 || out_b == 0 {
            return 1;
        }
        if ((src.wrapping_add(KIND)) as *const u8).read() != VOICE_KIND {
            return 1;
        }
        if lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, src) == 0 {
            return 1;
        }
        let h = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, src);
        let w = lf_checker_rt::callee_thiscall!(EVENT_CALLEE, u32, h, 0);
        (out_b as *mut u32).write_unaligned(w);
        // Scratch pair mirroring the original's zeroed argument slots.
        let frame = [out_a & 0xFFFF_FF00, out_b];
        let obj = if ((this.wrapping_add(SLOT_A)) as *const u32).read_unaligned() != 0 {
            ((this.wrapping_add(SLOT_A)) as *const u32).read_unaligned()
        } else {
            ((this.wrapping_add(SLOT_B)) as *const u32).read_unaligned()
        };
        let u = lf_checker_rt::callee_thiscall!(SOURCE_CALLEE, u32, obj, frame.as_ptr() as u32);
        (out_a as *mut u32).write_unaligned(u);
        (u & 0xFFFF_FF00) | 1
    }
});
