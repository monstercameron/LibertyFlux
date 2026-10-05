// original: 0x00b952a0 TERMINATE_ALL_SCRIPTS_WITH_THIS_NAME
/// Script native `TERMINATE_ALL_SCRIPTS_WITH_THIS_NAME`.
///
/// Forwards one script argument (a script-name hash) to the engine. No
/// return slot is written. (The original cleans its one pushed argument
/// with `(an instruction of the original)`; the effect on the stack pointer is identical to the
/// plain cdecl return here.)
export!(cdecl, rw_00b952a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
