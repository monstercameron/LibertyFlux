// original: 0x00a00450 APPLY_FORCE_TO_OBJECT
/// Native handler `APPLY_FORCE_TO_OBJECT`: apply a force to an object: pass the call context and the engine routine through to the shared native unpacker.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00a00450(ctx: u32) -> u32 {
    unsafe {
        // Context passthrough: the callee unpacks the arguments itself.
        // The pushed routine address is relocated like any code pointer.
        let ans = callee_cdecl!(1, u32, relocated(0x00A024F0), ctx);
        ans
    }
});
