// original: 0x00bd45f0 TRIGGER_PTFX_ON_VEH
/// Script native `TRIGGER_PTFX_ON_VEH` (hash 0x3C7B6092).
///
/// Passes a constant engine address plus the call context itself to the
/// shared particle-effect helper. No return slot is written.
///
/// The constant is a relocated image address (Verified: its instruction
/// immediate has a base-relocation entry, and the running original pushes
/// the relocated value), so it is derived from the worker's image base.
/// What it points to is Unknown.
export!(cdecl, rw_00bd45f0(ctx: *const u8) -> u32 {
    unsafe {
        /// Constant first word of the engine call, as a file VA (role unknown).
        const SHARED_PARAM: u32 = 0x00bd67d0;
        callee_cdecl!(1, u32, relocated(SHARED_PARAM), ctx as u32)
    }
});
