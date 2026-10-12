// original: 0x0098C180 NativeImpl_RETUNE_RADIO_UP_4

/// Tests the radio state word at `this + 0xB0C`. State 1 returns true; state
/// 2 calls the intercepted thiscall helper and tests its low byte; other
/// states return false. Only AL is changed after the helper, so its upper
/// return bytes are preserved, as are the upper bytes of other input states.
lf_checker_rt::export!(thiscall, rw_nativeimpl_retune_radio_up_4(this: u32) -> u32 {
    const RADIO_STATE: u32 = 0xB0C;
    const HELPER_RVA: u32 = 1;
    const AL_MASK: u32 = 0xFF;

    unsafe {
        let state = (this.wrapping_add(RADIO_STATE) as *const u32).read_unaligned();
        if state == 1 {
            return state;
        }
        if state != 2 {
            return state & !AL_MASK;
        }

        let helper: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(HELPER_RVA));
        let mut result = helper(this);
        result = (result & !AL_MASK) | u32::from(result as u8 != 0);
        result
    }
});
