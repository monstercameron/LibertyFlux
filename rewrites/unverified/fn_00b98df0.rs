// original: 0x00b98df0 NativeImpl_IS_GAME_KEYBOARD_NAV_UP_PRESSED

/// Returns whether the nav-up key is pressed, from RTX-remapped key state.
///
/// Same shape as `rw_00b98d30` with its own key bytes (`KEY_LO`, `KEY_HI`),
/// fallback code `CODE` and kind string `KIND`: enable flag at `FLAG_OFF`
/// gates an early 0, a zero caller byte selects the fallback input method
/// (bit 0 from its low byte), otherwise the xored key bytes decide against
/// 0x7F. Upper return bytes are the same residues as its siblings.
///
/// Original: 0x00B98DF0 (cdecl, one stack word, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b98df0(use_key: u32) -> u32 {
    unsafe {
        const STATE_CALLEE: u32 = 1;
        const FALLBACK_CALLEE: u32 = 2;
        const OBJ: u32 = 0x0118D110;
        const KIND: u32 = 0x00EB5D30;
        const CODE: u32 = 0xC8;
        const FLAG_OFF: u32 = 0x328D;
        const KEY_LO: u32 = 0x2EBE;
        const KEY_HI: u32 = 0x2EBC;
        const KEY_END: u32 = 0x2EB8;
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
