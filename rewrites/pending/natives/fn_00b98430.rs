// original: 0x00b98430 IS_AUTO_AIMING_ON
// IS_AUTO_AIMING_ON: engine(), zero-extend low byte into return slot.
export!(cdecl, rw_00b98430(ctx: u32) -> u32 {
    unsafe {
        let r = callee_cdecl!(1, u32,);
        *ret_of(ctx) = r & 0xFF;
        r
    }
});
