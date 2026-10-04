// original: 0x00ba0c30 REGISTER_HATED_TARGETS_AROUND_PED
/// Script native `REGISTER_HATED_TARGETS_AROUND_PED` (hash 0x70A62140).
///
/// Forwards 2 script arguments to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
/// No return slot is written.
export!(cdecl, rw_00ba0c30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
