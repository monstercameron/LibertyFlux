// original: 0x009cc3a0 REQUEST_MISSION_AUDIO_BANK
// REQUEST_MISSION_AUDIO_BANK: engine(arg0), low byte to return slot.
export!(cdecl, rw_009cc3a0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a);
        *ret_of(ctx) = r & 0xFF;
        r
    }
});
