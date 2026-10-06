// original: 0x00a01e40 SET_OBJECT_PROOFS
/// Script native `SET_OBJECT_PROOFS`.
///
/// Forwards 6 script arguments to the engine: argument 0 forwarded unchanged; argument 1 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 2 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 3 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 4 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 5 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00a01e40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            u32::from(*args.add(1) != 0),
            u32::from(*args.add(2) != 0),
            u32::from(*args.add(3) != 0),
            u32::from(*args.add(4) != 0),
            (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(5) != 0),
        )
    }
});
