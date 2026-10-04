// original: 0x00bc5d10 GET_CLOSEST_CAR
/// Script native `GET_CLOSEST_CAR` (hash 0x2CB303F8).
///
/// Reads six script arguments (four coordinates and two selectors), forwards all six words straight to the engine closest-car search (the float coordinates travel through vector registers and stack scratch but arrive in order), and stores the full engine answer into the return slot.
export!(cdecl, rw_00bc5d10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
