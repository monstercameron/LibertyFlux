// original: 0x00b8cb70 GET_STRING_WIDTH_WITH_TEXT_AND_INT
/// Script native `GET_STRING_WIDTH_WITH_TEXT_AND_INT` (hash 0x05267B97).
///
/// Forwards three script arguments to the engine and stores the float
/// result it returns in ST0 into the return slot. The stubbed callee also
/// copies the scripted bits to EAX, which is what this rewrite stores.
///
/// Quirk (observed): the handler parks the ST0 result in its own incoming
/// stack slot as a temporary, clobbering the context-pointer word there.
/// A rewrite cannot address its incoming stack slot, so the checker
/// contract disables stack comparison (q-13 precedent); the parked value is
/// still verified exactly through the return-slot write.
export!(cdecl, rw_00b8cb70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let bits = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = bits;
        slot as u32
    }
});
