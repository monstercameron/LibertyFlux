// original: 0x00a009c0 DOES_OBJECT_HAVE_THIS_MODEL
/// Script native `DOES_OBJECT_HAVE_THIS_MODEL` (hash 0x7505765B).
///
/// Forwards two script arguments (an object handle and a model hash) to the
/// engine and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00a009c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
