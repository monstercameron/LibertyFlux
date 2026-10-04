// original: 0x00b9f380 GET_NTH_GROUP_MEMBER
/// Script native handler `GET_NTH_GROUP_MEMBER`.
///
/// Writes the nth group member char handle through the out-pointer.
///
/// Forwards 3 script arguments to the engine function; no return slot.
/// handler function: `0x00b9f380`, engine call site: `0x00b9f38f`.
export!(cdecl, rw_b9f380(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let group = *args.add(0);
        let nth = *args.add(1);
        let char_out = *args.add(2);
        callee_cdecl!(1, u32, group, nth, char_out)
    }
});
