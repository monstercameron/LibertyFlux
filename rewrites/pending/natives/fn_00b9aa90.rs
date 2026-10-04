// original: 0x00b9aa90 GET_RANDOM_NETWORK_RESTART_NODE_EXCLUDING_GROUP
/// Script native `GET_RANDOM_NETWORK_RESTART_NODE_EXCLUDING_GROUP`
/// (hash 0x00393309).
///
/// Passes two values to the engine: the address of an engine callback
/// function, held as an immediate operand in the original, and the script
/// call context pointer itself. The handler reads no script arguments. No
/// return slot is written.
///
/// The immediate has a relocation entry, so it tracks the image base; the
/// address is derived with `relocated` like any other reference. The callee
/// is stubbed by the checker, so the address is compared as a datum and
/// never executed. A future integration must rewire this to the recompiled
/// callback symbol.
export!(cdecl, rw_00b9aa90(ctx: *const u8) -> u32 {
    /// File VA of the callback the original pushes as an immediate.
    const CALLBACK_FILE_VA: u32 = 0x00B9_C920;
    callee_cdecl!(1, u32, relocated(CALLBACK_FILE_VA), ctx as u32)
});
