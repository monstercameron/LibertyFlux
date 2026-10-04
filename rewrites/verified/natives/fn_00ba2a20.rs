// original: 0x00ba2a20 UPDATE_PED_PHYSICAL_ATTACHMENT_POSITION
/// Script native `UPDATE_PED_PHYSICAL_ATTACHMENT_POSITION` (hash 0x10A62603).
///
/// Forwards seven script arguments (a ped handle followed by six float
/// bit-patterns forming two coordinate triples) to the engine as seven
/// plain words. The original shuffles the words through two inline stack
/// buffers, but the observable call is a straight 1:1 forward; the buffers
/// are an implementation detail. No return slot is written.
export!(cdecl, rw_00ba2a20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
        )
    }
});
