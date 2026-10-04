// original: 0x005e8170 ADD_TO_HTML_SCRIPT_OBJECT
/// Script native `ADD_TO_HTML_SCRIPT_OBJECT` (hash 0x3ECC0086).
///
/// Forwards two script arguments to the engine worker.
/// The engine callee cleans the stack itself (no caller cleanup), so it
/// is called with the stdcall convention. No return slot is written.
export!(cdecl, rw_005e8170(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_stdcall!(1, u32, *args, *args.add(1))
    }
});
