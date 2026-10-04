// original: 0x009671B0 refresh_system_metrics
/// Query system parameter 0x11 and return the result.
///
/// Calls `SystemParametersInfoA` with the action `0x11` and three zero
/// arguments, all pushed as constants, and returns its `BOOL` result.
/// The import is intercepted by the checker (IAT entry), so the rewrite
/// performs the same stdcall through the stub table with the same
/// constant arguments.
///
/// Original: 0x009671B0 (cdecl, no stack words; the callee is stdcall).

export!(cdecl, rw_009671B0() -> u32 {
    callee_stdcall!(1, u32, 0x11, 0, 0, 0)
});
