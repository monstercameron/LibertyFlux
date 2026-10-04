// original: 0x00e4af00 UISelectMenu::vf110

/// Menu-select tick: poll the input reader, refresh the highlighted entry,
/// then dispatch on the fetched select descriptor.
///
/// `this` is the menu object. Selection state lives at `+0x208` (with a
/// mirror copy at `+0x20c`), the current mode at `+0x1e8`, and a fallback
/// value at `+0x204`. No stack arguments; thiscall.
///
/// Behaviour: fetch a 12-byte descriptor through the helper (callee 0) and
/// keep its first word. Probe the 7-argument input reader (callee 1) twice
/// with first arguments 3 then 1: if either probe is true, refresh via
/// callee 2, store its result to both selection slots, acknowledge through
/// callee 3 and notify through callee 4. Repeat with first arguments 2 then
/// 0, refreshing via callee 5 instead. Then dispatch on the descriptor word:
/// 5, or 0x16 with a confirming probe (first argument 0xc), runs the shared
/// finish path (callee 7 then callee 3) and returns callee 3's answer; any
/// other value is returned as is; 0x16 without confirmation either applies
/// the configured value (callee 6) when the mode is 8, or copies the
/// fallback into the selection slot and tails to callee 7.
///
/// Edge cases: all-false probes skip both refresh blocks; a descriptor word
/// other than 5/0x16 returns immediately; the tail path returns callee 7's
/// answer. Only the low byte of each probe answer is tested.
lf_checker_rt::export!(thiscall, rw_00e4af00(this: u32) -> u32 {
    unsafe {
        const HELPER_THIS: u32 = 0x019D_2E08;
        const QUERY_CODE: u32 = 0x0040_0F04;
        const NOTIFY_THIS: u32 = 0x0117_6888;
        const NOTE_A: u32 = 0x00F1_7270;
        const NOTE_B: u32 = 0x00F1_7288;
        const SEL_VALUE: u32 = 0x208;
        const SEL_MIRROR: u32 = 0x20C;
        const MODE_FIELD: u32 = 0x1E8;
        const FALLBACK_VALUE: u32 = 0x204;
        const DESC_SELECT: u32 = 5;
        const DESC_CONFIG: u32 = 0x16;
        const MODE_DIRECT: u32 = 8;
        const CONFIG_ARG: u32 = 0x35;
        const POLL: u32 = 1;
        const DESC_CALLEE: u32 = 0;
        const PROBE_CALLEE: u32 = 1;
        const REFRESH_A: u32 = 2;
        const ACK_CALLEE: u32 = 3;
        const NOTIFY_CALLEE: u32 = 4;
        const REFRESH_B: u32 = 5;
        const CONFIG_CALLEE: u32 = 6;
        const FINISH_CALLEE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn probe(a0: u32, a4: u32) -> bool {
            (lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, a0, 1, 0, 0, a4, 0, 0) & 0xff) != 0
        }
        #[inline(always)]
        unsafe fn refresh(this: u32, id: u32, note: u32) {
            unsafe {
                let v = lf_checker_rt::callee_thiscall!(id, u32, this);
                wr32(this + SEL_VALUE, v);
                wr32(this + SEL_MIRROR, v);
                lf_checker_rt::callee_thiscall!(
                    ACK_CALLEE,
                    u32,
                    lf_checker_rt::relocated(HELPER_THIS),
                    POLL
                );
                lf_checker_rt::callee_thiscall!(
                    NOTIFY_CALLEE,
                    u32,
                    lf_checker_rt::relocated(NOTIFY_THIS),
                    lf_checker_rt::relocated(note)
                );
            }
        }
        #[inline(always)]
        unsafe fn finish(this: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, this);
                lf_checker_rt::callee_thiscall!(
                    ACK_CALLEE,
                    u32,
                    lf_checker_rt::relocated(HELPER_THIS),
                    POLL
                )
            }
        }

        let desc = lf_checker_rt::callee_thiscall!(
            DESC_CALLEE,
            u32,
            lf_checker_rt::relocated(HELPER_THIS),
            QUERY_CODE, // opaque query code, not an address (no reloc entry)
            POLL
        );
        let word = rd32(desc);
        if probe(3, 1) || probe(1, 1) {
            refresh(this, REFRESH_A, NOTE_A);
        }
        if probe(2, 1) || probe(0, 1) {
            refresh(this, REFRESH_B, NOTE_B);
        }
        if word == DESC_SELECT {
            return finish(this);
        }
        if word != DESC_CONFIG {
            return word;
        }
        if probe(0x0c, 0) {
            return finish(this);
        }
        if rd32(this + MODE_FIELD) != MODE_DIRECT {
            wr32(this + SEL_VALUE, rd32(this + FALLBACK_VALUE));
            return lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, this);
        }
        lf_checker_rt::callee_cdecl!(CONFIG_CALLEE, u32, CONFIG_ARG)
    }
});
