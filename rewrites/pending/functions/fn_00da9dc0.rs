// original: 0x00da9dc0 flee_window_check
/// Check whether the approach window is open for the candidate.
///
/// Runs a chain of gates: the candidate must be non-null with mode bit 1
/// clear (mode bit 0 additionally requires the flag word's approval), the
/// biased sample must exceed the candidate's tracked value, and the looked-up
/// record must either be absent or stale, or carry no veto byte matching the
/// candidate's word. Returns 1 when every gate passes, 0 otherwise.
export!(cdecl, rw_00da9dc0(x: u32, s: u32, t: u32, f: f32) -> u32 {
    unsafe {
        /// Candidate's link word, flag byte and tracked value/word.
        const LINK: u32 = 0x00;
        const MODE: u32 = 0x4b;
        const TRACKED: u32 = 0x18;
        const WORD: u32 = 0x52;
        /// Flag word gating mode bit 0.
        const FLAGW: u32 = 0x29c;
        /// Reference object's value offset and the sample bias.
        const REF: u32 = 0x38;
        const BIAS: f32 = 0.1;
        /// Record kind/state words and veto bytes.
        const KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0x80;
        const STATE: u32 = 0x1304;
        const STATE_WANT: u32 = 4;
        const VETO0: u32 = 0x1ef0;
        const VETO1: u32 = 0x1ef1;
        let c0 = ((s.wrapping_add(LINK)) as *const u32).read_unaligned();
        if c0 == 0 {
            return 0;
        }
        let mode = (*((s.wrapping_add(MODE)) as *const u8));
        if mode & 2 != 0 {
            return 0;
        }
        if mode & 1 != 0 {
            let xw: u32 = ((x.wrapping_add(FLAGW)) as *const u32).read_unaligned();
            if xw & 0x800 == 0 && xw as u8 & 4 == 0 {
                return 0;
            }
        }
        let sample = f - BIAS;
        let tracked = f32::from_bits(((s.wrapping_add(TRACKED)) as *const u32).read_unaligned()) - f32::from_bits(((t.wrapping_add(REF)) as *const u32).read_unaligned());
        // The original's comiss+jbe fails unordered results too.
        if !(sample > tracked) {
            return 0;
        }
        let d: u32 = callee_cdecl!(1, u32, c0);
        if d == 0 {
            return 1;
        }
        if ((d.wrapping_add(KIND)) as *const u32).read_unaligned() & KIND_MASK != KIND_WANT {
            return 1;
        }
        if ((d.wrapping_add(STATE)) as *const u32).read_unaligned() != STATE_WANT {
            return 1;
        }
        let want = ((s.wrapping_add(WORD)) as *const u16).read_unaligned() as i32;
        if (*((d.wrapping_add(VETO0)) as *const u8)) as i8 as i32 == want {
            return 0;
        }
        if (*((d.wrapping_add(VETO1)) as *const u8)) as i8 as i32 == want {
            return 0;
        }
        1
    }
});
