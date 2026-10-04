// original: 0x00ba0290 IS_RELATIONSHIP_SET
/// Script native `IS_RELATIONSHIP_SET` (hash 0x4C076B40).
///
/// Forwards three group/relationship indexes to the engine and stores the
/// low byte of its answer (zero-extended) into the return slot. The value
/// left in EAX on exit is the return-slot pointer, reproduced here.
export!(cdecl, rw_00ba0290(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
