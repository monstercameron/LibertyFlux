// original: 0x00b98d30 NativeImpl_IS_GAME_KEYBOARD_NAV_LEFT_PRESSED

/// Returns whether the nav-left key is pressed, from RTX-remapped key state.
///
/// Reads the shared key-state block from `STATE_CALLEE` (called with 2).
/// When the block's enable flag at `FLAG_OFF` is clear the answer is 0. When
/// the caller passes a zero byte the answer comes from the fallback input
/// method `FALLBACK_CALLEE` on `OBJ` with (`CODE`, 1, `KIND`), normalized to
/// bit 0 from the callee's low byte. Otherwise two state bytes at `KEY_LO`
/// and `KEY_HI` are xored and the answer is 1 when the result exceeds 0x7F.
///
/// The returned upper bytes are residue, reproduced exactly: the state
/// pointer (early-out), the fallback's upper bytes, or the state pointer
/// plus `KEY_END` (main path). The pushed addresses carry relocations, so
/// both sides push relocated addresses compared as image-relative offsets.
///
/// Original: 0x00B98D30 (cdecl, one stack word, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b98d30(use_key: u32) -> u32 {
    unsafe {
        const STATE_CALLEE: u32 = 1;
        const FALLBACK_CALLEE: u32 = 2;
        const OBJ: u32 = 0x0118D110;
        const KIND: u32 = 0x00EB5D80;
        const CODE: u32 = 0xCB;
        const FLAG_OFF: u32 = 0x328D;
        const KEY_LO: u32 = 0x2EDE;
        const KEY_HI: u32 = 0x2EDC;
        const KEY_END: u32 = 0x2ED8;
        const THRESH: u8 = 0x7F;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let st: u32 = lf_checker_rt::callee_cdecl!(STATE_CALLEE, u32, 2);
        if rd8(st + FLAG_OFF) == 0 {
            return st & 0xFFFF_FF00;
        }
        if use_key & 0xFF == 0 {
            let r: u32 = lf_checker_rt::callee_thiscall!(
                FALLBACK_CALLEE,
                u32,
                lf_checker_rt::relocated(OBJ),
                CODE,
                1,
                lf_checker_rt::relocated(KIND)
            );
            return (r & 0xFFFF_FF00) | u32::from((r & 0xFF) != 0);
        }
        let st2: u32 = lf_checker_rt::callee_cdecl!(STATE_CALLEE, u32, 2);
        let cl = rd8(st2 + KEY_LO) ^ rd8(st2 + KEY_HI);
        let upper = st2.wrapping_add(KEY_END) & 0xFFFF_FF00;
        if cl > THRESH { upper | 1 } else { upper }
    }
});
