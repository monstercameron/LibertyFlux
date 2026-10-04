// original: 0x00b8c850 GET_FIRST_BLIP_INFO_ID
/// Script native `GET_FIRST_BLIP_INFO_ID` (hash 0x3BD729E9).
///
/// Forwards a blip handle to the engine and stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00b8c850(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args.add(0));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
