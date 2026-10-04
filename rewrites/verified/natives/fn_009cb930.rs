// original: 0x009cb930 ADD_LINE_TO_MOBILE_PHONE_CALL
/// Script native `ADD_LINE_TO_MOBILE_PHONE_CALL` (hash 0x0BED1DDE).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_009cb930(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
