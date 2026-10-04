// original: 0x009cbe40 IS_MOBILE_PHONE_RADIO_ACTIVE
/// Script native `IS_MOBILE_PHONE_RADIO_ACTIVE` (hash 0x4AF14146).
///
/// Calls the engine phone-radio query with no arguments and stores the low
/// byte of the answer (a boolean) into the return slot.
export!(cdecl, rw_009cbe40(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        slot as u32
    }
});
